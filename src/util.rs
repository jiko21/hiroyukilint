pub mod util {
    use std::io::BufRead;
    pub fn read_file_to_array<R: BufRead>(buf: R) -> Vec<String> {
        buf.lines()
            .map(|l| l.expect("cannot parse text line"))
            .filter(|l| !l.starts_with('#'))
            .collect()
    }
}
