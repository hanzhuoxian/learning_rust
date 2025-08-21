use test_integration::add;

mod common;

#[test]
fn adder() {
    common::setup();
    add(1,3);
}