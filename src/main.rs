use clap::Parser;
use windows::Win32::System::ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
use windows::Win32::System::Threading::GetCurrentProcess;

#[derive(clap::Parser)]
struct Cli {
    library: Library,
    input_path: std::path::PathBuf,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum Library {
    None,
    Libsais,
    Libsais64,
    LibsaisOpenMP,
    Libsais64OpenMP,
    Divsufsort,
    Suffix,
    Bio,
    Psacak,
    SaisDrum,
}

fn main() {
    // set num threads for psacak
    rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();

    let cli = Cli::parse();

    let mut text = std::fs::read(&cli.input_path).unwrap();

    text.truncate(1_999_999_999);

    // rust-bio needs the sentinel at the end of the text
    text.push(0);

    println!("Text length: {}", text.len());

    let text_str = std::str::from_utf8(&text).unwrap();

    let start = std::time::Instant::now();
    let last = match cli.library {
        Library::Libsais => run_libsais_single_threaded(&text),
        Library::Libsais64 => run_libsais64_single_threaded(&text),
        Library::LibsaisOpenMP => run_libsais_multi_threaded(&text),
        Library::Libsais64OpenMP => run_libsais64_multi_threaded(&text),
        Library::Divsufsort => run_divsufsort(&text),
        Library::Suffix => run_suffix(text_str),
        Library::Bio => run_bio(&text),
        Library::Psacak => run_psacak(&text),
        Library::SaisDrum => run_sais_drum(&text),
        Library::None => 0,
    };

    let handle = unsafe { GetCurrentProcess() };
    let mut memory_info = PROCESS_MEMORY_COUNTERS::default();
    let ptr: *mut PROCESS_MEMORY_COUNTERS = &mut memory_info;
    unsafe {
        GetProcessMemoryInfo(handle, ptr, std::mem::size_of_val(&memory_info) as u32).unwrap()
    };

    println!("Last suffix index: {}", last);
    println!("Elapsed time: {} seconds", start.elapsed().as_secs());
    println!("Peak memory usage: {}", memory_info.PeakWorkingSetSize);
}

fn run_libsais_single_threaded(text: &[u8]) -> i32 {
    libsais::SuffixArrayConstruction::for_text(text)
        .in_owned_buffer32()
        .run()
        .expect("libsais single threaded")
        .into_vec()
        .last()
        .unwrap()
        .to_owned()
}

fn run_libsais64_single_threaded(text: &[u8]) -> i32 {
    libsais::SuffixArrayConstruction::for_text(text)
        .in_owned_buffer64()
        .run()
        .expect("libsais single threaded")
        .into_vec()
        .last()
        .unwrap()
        .to_owned() as i32
}

fn run_libsais_multi_threaded(text: &[u8]) -> i32 {
    libsais::SuffixArrayConstruction::for_text(text)
        .in_owned_buffer32()
        .multi_threaded(libsais::ThreadCount::fixed(8))
        .run()
        .expect("libsais multi threaded")
        .into_vec()
        .last()
        .unwrap()
        .to_owned()
}

fn run_libsais64_multi_threaded(text: &[u8]) -> i32 {
    libsais::SuffixArrayConstruction::for_text(text)
        .in_owned_buffer64()
        .multi_threaded(libsais::ThreadCount::fixed(8))
        .run()
        .expect("libsais64 multi threaded")
        .into_vec()
        .last()
        .unwrap()
        .to_owned() as i32
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

fn run_sais_drum(text: &[u8]) -> i32 {
    sais_drum::SaisBuilder::new()
        .construct_suffix_array(text)
        .last()
        .unwrap()
        .to_owned() as i32
}
