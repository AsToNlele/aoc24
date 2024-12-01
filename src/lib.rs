use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_lines(str: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(str)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    lines.collect()
}

pub trait ToInt {
    fn to_int(&self) -> u32;
}

impl ToInt for String {
    fn to_int(&self) -> u32 {
        self.parse::<u32>().unwrap_or(0)
    }
}

impl ToInt for &str {
    fn to_int(&self) -> u32 {
        self.parse::<u32>().unwrap_or(0)
    }
}
