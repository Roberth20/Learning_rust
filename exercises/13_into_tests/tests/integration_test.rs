/* This is a integration test, they test the public API of the library and how it
 work together.

To run integration test only,
* cargo test --test <integration_test_name>

*/
// Bring to scope the crate to test
use into_tests::add_two;

// Bring to scope teh setup func
mod common;

// Test as normal
#[test]
fn it_adds_two() {
    let user = common::setup();
    assert_eq!(add_two(2), 4);
}