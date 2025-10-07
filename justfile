# for windows
set shell := ["powershell.exe", "-c"]

run input_path:
    cargo run --release libsais "{{input_path}}"
    cargo run --release libsais64 "{{input_path}}"
    cargo run --release libsais-open-mp "{{input_path}}"
    cargo run --release libsais-open-mp64 "{{input_path}}"
    cargo run --release divsufsort "{{input_path}}"
    cargo run --release suffix "{{input_path}}"
    cargo run --release bio "{{input_path}}"
    cargo run --release psacak "{{input_path}}"
    cargo run --release psacak-threads "{{input_path}}"
    cargo run --release sais-drum "{{input_path}}"
    cargo run --release sais-drum64 "{{input_path}}"
    cargo run --release sufr-partial128 "{{input_path}}"
