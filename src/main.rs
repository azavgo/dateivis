use std::fs::File;
use std::io::{BufReader, Read}; 

#[derive(Debug)]
pub enum DateivisError { 
    IOError(std::io::Error),
    FileLengthError(String),  
}

impl From<std::io::Error> for DateivisError {
    fn from(error: std::io::Error) -> Self {
        DateivisError::IOError(error)
    }
}

#[derive(Debug, PartialEq)]
pub struct Point {
    x: u8, 
    y: u8, 
}

impl Point {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x: x, y: y }
    }
}

fn main() -> Result<(), DateivisError> {
    let file = File::open("LICENSE")?;
    
    let mut buf_reader = BufReader::new(file); 
    let mut content = Vec::new(); 
    buf_reader.read_to_end(&mut content)?;
    if content.len() % 2 != 0 {
        content.push(0); 
    }



    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_new() {
        let point = Point::new(22, 132); 
        assert_eq!(Point{x: 22, y: 132}, point);
    }
}