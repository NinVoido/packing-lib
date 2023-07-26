use crate::math_utils::Point;

use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

fn parse_unchecked(s: &str) -> f64 {
    return s.parse().unwrap();
}

pub fn load_file(path: String) -> Result<Vec<Point>, Error> {
    let path = Path::new(&path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut res = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(s) => {
                let cords: Vec<f64> = s.split(" ").map(parse_unchecked).collect();
                res.push(Point::new(cords));
            }
            Err(e) => return Err(e),
        }
    }

    Ok(res)
}
