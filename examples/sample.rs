use serde::Deserialize;
use test_csv::test_csv;

#[derive(Default, Deserialize)]
struct Data {
    a: String,
    b: u32,
    c: u32,
}

#[test_csv("examples/file.csv")]
fn foo(data: Data) {
    println!("a: {}, b: {}, c: {}", data.a, data.b, data.c);
    assert!(data.b > data.c);
}

fn main() {}
