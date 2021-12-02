pub mod maths;

use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum InputError {
    IOError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
}

impl From<std::io::Error> for InputError {
    fn from(e: std::io::Error) -> InputError {
        InputError::IOError(e)
    }
}

impl From<std::num::ParseIntError> for InputError {
    fn from(e: std::num::ParseIntError) -> InputError {
        InputError::ParseIntError(e)
    }
}

pub fn get_parsed_lines_stdin<T>() -> Result<Vec<T>, InputError>
where
    T: std::str::FromStr,
    InputError: From<<T as std::str::FromStr>::Err>,
{
    get_parsed_lines(io::stdin().lock())
}

pub fn get_parsed_lines<T, R>(reader: R) -> Result<Vec<T>, InputError>
where
    R: BufRead,
    T: std::str::FromStr,
    InputError: From<<T as std::str::FromStr>::Err>,
{
    reader
        .lines()
        .map(|line| -> Result<T, InputError> { Ok(line?.parse::<T>()?) })
        .collect()
}

/*
pub fn get_integer_csv<T>() -> Result<Vec<T>, InputError>
where
    T: std::str::FromStr,
    InputError: From<<T as std::str::FromStr>::Err>,
{
    let stdin = io::stdin();
    stdin
        .lock()
        .lines()
        .map(|line| -> Result<std::str::Split<_>, InputError> { Ok(line?.split(',')) })
        .map(|split| -> Result<_, InputError> {
            Ok(split?.map(|s| -> Result<_, InputError> { Ok(s.parse::<T>()?) } ).and_then(|s| s.collect())
        )})
        //.map(|split| -> Result<_, InputError> { Ok(split?.collect())})
        //.map(|split: Result<, InputError>| -> Result<T, InputError> { Ok(split?.collect())})
        .collect()

}
*/
