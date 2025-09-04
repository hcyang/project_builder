mkdir poc_quick_sort
cd poc_quick_sort
cargo init --bin
mkdir src
copy ..\poc_algorithm_builder_rust_quick_sort_generated_program_payload.rs src\main.rs
copy ..\poc_algorithm_builder_rust_quick_sort_generated_program_payload_tests.rs src\main_test.rs
cargo build
cargo test