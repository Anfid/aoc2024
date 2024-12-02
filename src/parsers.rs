pub fn u64_from_ascii(bytes: &[u8]) -> u64 {
    bytes
        .iter()
        .filter(|byte| byte.is_ascii_digit())
        .fold(0u64, |acc, &d| acc * 10 + (d - b'0') as u64)
}

pub trait BytesAsciiExt {
    fn ascii_lines(&self) -> impl Iterator<Item = &[u8]>;

    fn ascii_words(&self) -> impl Iterator<Item = &[u8]>;
}

impl BytesAsciiExt for &[u8] {
    fn ascii_lines(&self) -> impl Iterator<Item = &[u8]> {
        self.split_inclusive(|&c| c == b'\n')
            .map(|l| l.trim_ascii_end())
    }

    fn ascii_words(&self) -> impl Iterator<Item = &[u8]> {
        self.split(|&c| c == b' ')
    }
}
