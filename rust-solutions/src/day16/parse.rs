use std::str::FromStr;
use std::error::Error;
use core::result;

use lazy_static::lazy_static;
use regex::Regex;


macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

type Result<T> = result::Result<T, Box<Error>>;

