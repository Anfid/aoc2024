pub fn u64_from_ascii(bytes: &[u8]) -> u64 {
    bytes
        .iter()
        .filter(|byte| byte.is_ascii_digit())
        .fold(0u64, |acc, &d| acc * 10 + (d - b'0') as u64)
}

macro_rules! num_from_digits {
    ($t:ty, $($d:tt),+) => {
        num_from_digits!(@internal [0], $t, $($d),+) - num_from_digits!(@internal_ones $($d),+) * b'0' as $t
    };

    (@internal [$res:tt], $t:ty, $last:tt) => {
        ($res) * 10 + $last as $t
    };
    (@internal [$res:tt], $t:ty, $head:tt, $($tail:tt),+) => {
        num_from_digits!(@internal [(($res) * 10 + $head as $t)], $t, $($tail),+)
    };

    (@internal_ones $x:tt) => {
        1
    };
    (@internal_ones $x:tt, $($xs:tt),+) => {
        1 + 10 * num_from_digits!(@internal_ones $($xs),+)
    };
}
pub(crate) use num_from_digits;

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
