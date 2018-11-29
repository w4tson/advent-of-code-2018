pub mod file;

pub fn to_uint(s : char) -> u32 {
    s.to_digit(10).unwrap()
}

