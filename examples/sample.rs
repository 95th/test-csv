fn main() {}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use test_csv::test_csv;

    #[derive(Deserialize)]
    struct Data {
        a: String,
        b: u64,
        c: u64,
    }

    #[test_csv("file.csv")]
    fn foo(d: Data) {
        assert!(d.b > d.c, "Failure on line: {} for {}", line_no(), d.a);
    }
}
