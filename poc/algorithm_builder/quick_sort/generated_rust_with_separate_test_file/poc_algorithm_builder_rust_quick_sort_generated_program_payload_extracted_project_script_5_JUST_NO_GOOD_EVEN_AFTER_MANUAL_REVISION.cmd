mkdir poc_algorithm_builder_rust_quick_sort_generated_program
cd poc_algorithm_builder_rust_quick_sort_generated_program
cargo init
mkdir src\tests
copy ..\poc_algorithm_builder_rust_quick_sort_generated_program_payload.rs src\
copy ..\poc_algorithm_builder_rust_quick_sort_generated_program_payload_tests.rs src\tests\
echo 'mod poc_algorithm_builder_rust_quick_sort_generated_program_payload;' > src\lib.rs
echo 'mod tests {' > src\tests\mod.rs
echo '    #[path = "poc_algorithm_builder_rust_quick_sort_generated_program_payload_tests.rs"] mod poc_algorithm_builder_rust_quick_sort_generated_program_payload_tests;' >> src\tests\mod.rs
echo '}' >> src\tests\mod.rs
cargo test