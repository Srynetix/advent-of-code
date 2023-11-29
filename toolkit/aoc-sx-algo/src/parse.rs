use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_lines<T>(data: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    data.lines()
        .map(|x| x.parse().expect("Could not parse lines from input data."))
        .collect()
}

pub fn parse_str_lines(data: &str) -> Vec<&str> {
    data.lines().collect()
}
