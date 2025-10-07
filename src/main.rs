use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;

use clap::Parser;
use serde::{Deserialize, Serialize};
#[derive(Debug, clap::Parser)]
struct Cli {
    library: Library,
    input_path: std::path::PathBuf,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum, PartialEq, Eq, Hash, Deserialize, Serialize)]
enum Library {
    None,
    Libsais,
    Libsais64,
    LibsaisOpenMp,
    LibsaisOpenMp64,
    Divsufsort,
    Suffix,
    Bio,
    Psacak,
    PsacakThreads,
    SaisDrum,
    SaisDrum64,
    SufrPartial128,
}

#[derive(Deserialize, Serialize)]
struct BenchmarkResult {
    elapsed_time_secs: f64,
    peak_memory_usage_gb: f64,
}

const SUFR_FILE_PATH: &str = "temp.sufr";
const RESULTS_FILE_PATH: &str = "results.json";

fn main() {
    let cli = Cli::parse();

    let mut text = vec![0; 1_999_999_999];
    let mut file = File::open(&cli.input_path).unwrap();
    file.read_exact(&mut text).unwrap();
    // rust-bio needs the sentinel at the end of the text
    text.push(0);
    let text_str = std::str::from_utf8(&text).unwrap();

    let rayon_num_threads = match cli.library {
        Library::PsacakThreads | Library::SufrPartial128 => 8,
        _ => 1,
    };

    rayon::ThreadPoolBuilder::new()
        .num_threads(rayon_num_threads)
        .build_global()
        .unwrap();

    let start = std::time::Instant::now();

    let last = match cli.library {
        Library::Libsais => run_libsais_single_threaded::<i32>(&text),
        Library::Libsais64 => run_libsais_single_threaded::<i64>(&text),
        Library::LibsaisOpenMp => run_libsais_multi_threaded::<i32>(&text),
        Library::LibsaisOpenMp64 => run_libsais_multi_threaded::<i64>(&text),
        Library::Divsufsort => run_divsufsort(&text),
        Library::Suffix => run_suffix(text_str),
        Library::Bio => run_bio(&text),
        Library::Psacak | Library::PsacakThreads => run_psacak(&text),
        Library::SaisDrum => run_sais_drum::<u32>(&text),
        Library::SaisDrum64 => run_sais_drum::<usize>(&text),
        Library::SufrPartial128 => run_sufr(text, 128),
        Library::None => 0,
    };

    println!("Last suffix index: {}", last);

    let result = BenchmarkResult {
        elapsed_time_secs: start.elapsed().as_millis() as f64 / 1_000.0,
        peak_memory_usage_gb: process_peak_memory_usage_gb() - 2.0,
    };

    println!("Elapsed time: {} seconds", result.elapsed_time_secs);
    println!("Peak memory usage: {} GB", result.peak_memory_usage_gb);

    let mut results = if fs::exists(RESULTS_FILE_PATH).unwrap() {
        let file = File::open(RESULTS_FILE_PATH).unwrap();
        serde_json::from_reader(file).unwrap()
    } else {
        HashMap::<Library, BenchmarkResult>::new()
    };

    results.insert(cli.library, result);

    let file = File::create(RESULTS_FILE_PATH).unwrap();
    serde_json::to_writer_pretty(file, &results).unwrap();

    if cli.library == Library::SufrPartial128 {
        fs::remove_file(SUFR_FILE_PATH).unwrap();
    }
}

fn run_libsais_single_threaded<I: libsais::IsValidOutputFor<u8>>(text: &[u8]) -> i32 {
    let &res = libsais::SuffixArrayConstruction::for_text(text)
        .in_owned_buffer::<I>()
        .single_threaded()
        .run()
        .expect("libsais single threaded")
        .into_vec()
        .last()
        .unwrap();

    <i32 as num_traits::NumCast>::from(res).unwrap()
}

fn run_libsais_multi_threaded<I: libsais::IsValidOutputFor<u8>>(text: &[u8]) -> i32 {
    let &res = libsais::SuffixArrayConstruction::for_text(text)
        .in_owned_buffer::<I>()
        .multi_threaded(libsais::ThreadCount::fixed(8))
        .run()
        .expect("libsais multi threaded")
        .into_vec()
        .last()
        .unwrap();

    <i32 as num_traits::NumCast>::from(res).unwrap()
}

fn run_divsufsort(text: &[u8]) -> i32 {
    divsufsort::sort(text)
        .into_parts()
        .1
        .last()
        .unwrap()
        .to_owned()
}

fn run_suffix(text: &str) -> i32 {
    suffix::SuffixTable::new(text)
        .table()
        .last()
        .unwrap()
        .to_owned() as i32
}

fn run_bio(text: &[u8]) -> i32 {
    bio::data_structures::suffix_array::suffix_array(text)
        .last()
        .unwrap()
        .to_owned() as i32
}

fn run_psacak(text: &[u8]) -> i32 {
    psacak::psacak(text).last().unwrap().to_owned() as i32
}

fn run_sais_drum<I: sais_drum::IndexStorage>(text: &[u8]) -> i32 {
    sais_drum::SaisBuilder::<_, I>::new()
        .construct_suffix_array(text)
        .last()
        .unwrap()
        .to_owned()
        .as_() as i32
}

fn run_sufr(text: Vec<u8>, max_query_len: usize) -> i32 {
    use libsufr::{suffix_array::SuffixArray, types::SufrBuilderArgs};

    let builder_args = SufrBuilderArgs {
        text,
        path: Some(SUFR_FILE_PATH.to_string()),
        low_memory: true,
        max_query_len: Some(max_query_len),
        is_dna: false,
        allow_ambiguity: false,
        ignore_softmask: false,
        sequence_starts: vec![0],
        sequence_names: vec![String::from("seq")],
        num_partitions: 1024,
        seed_mask: None,
        random_seed: 42,
    };

    SuffixArray::write(builder_args).unwrap();

    // the return values are there to make sure that nothing gets optimized away.
    // not needed here, because a file is written.
    0
}

// ---------- just for fun, I implemented the memory usage functionaliy by hand ----------
#[cfg(windows)]
fn process_peak_memory_usage_gb() -> f64 {
    use windows::Win32::System::ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
    use windows::Win32::System::Threading::GetCurrentProcess;

    let handle = unsafe { GetCurrentProcess() };
    let mut memory_info = PROCESS_MEMORY_COUNTERS::default();
    let ptr: *mut PROCESS_MEMORY_COUNTERS = &mut memory_info;
    // safety: standard usage of this windows API, I think it should be safe
    unsafe {
        GetProcessMemoryInfo(handle, ptr, std::mem::size_of_val(&memory_info) as u32).unwrap()
    };

    memory_info.PeakWorkingSetSize as f64 / 1_000_000_000.0
}

#[cfg(unix)]
fn process_peak_memory_usage_gb() -> f64 {
    let mut memory_info: libc::rusage = unsafe { std::mem::zeroed() };
    let ret =
        unsafe { libc::getrusage(libc::RUSAGE_SELF, (&mut memory_info) as *mut libc::rusage) };
    assert!(ret == 0);

    memory_info.ru_maxrss as f64 / 1_000_000.0
}
