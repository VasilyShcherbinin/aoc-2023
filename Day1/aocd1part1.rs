use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("C:\\Users\\vasil\\Desktop\\input.txt")?;
    let reader = BufReader::new(file);

    let regex_first_number = Regex::new(r"^(?:[a-zA-Z]+)?(\d)").unwrap();
    let regex_last_number = Regex::new(r"(\d)(?:[a-zA-Z]+)?$").unwrap();

    let sum: i32 = reader
    .lines()
    .map(|line| {
        let line_string = line.unwrap();
        if let (Some(capture1), Some(capture2)) = (
            regex_first_number.captures(&line_string),
            regex_last_number.captures(&line_string),
        ) {
            let result = format!("{}{}", &capture1[1], &capture2[1]);
            println!("{}", result);
            result.parse::<i32>().unwrap_or(0)
        } else {
            0
        }
    })
    .sum();


    print!("{}", sum);
    Ok(())
}
