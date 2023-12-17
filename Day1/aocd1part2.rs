use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use fancy_regex::Regex as FancyRegex;

fn main() -> io::Result<()> {
    let file = File::open("C:\\Users\\vasil\\Desktop\\input.txt")?;
    let reader = BufReader::new(file);

    let regex_first_number = Regex::new(r"^(?:[a-zA-Z]+)?(\d)").unwrap();
    let regex_last_number = Regex::new(r"(\d)(?:[a-zA-Z]+)?$").unwrap();
    let regex_convert = FancyRegex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine))").unwrap();

    let mut sum = 0;

    for line in reader.lines() {
        let line_string = line.unwrap();
        print!("{}:",line_string);
        let mut modified_line = line_string.to_string(); // Create a new string to store modifications

        for cap in regex_convert.captures_iter(&line_string) {
            if let Some(matched) = cap.as_ref().ok().and_then(|c| c.get(0)) {
                let replacement = match matched.as_str() {
                    "one" => "1",
                    "two" => "2",
                    "three" => "3",
                    "four" => "4",
                    "five" => "5",
                    "six" => "6",
                    "seven" => "7",
                    "eight" => "8",
                    "nine" => "9",
                    _ => continue, // Skip if no match
                };
                modified_line = modified_line.replace(matched.as_str(), replacement);
            }
        }

        print!("{}:",modified_line);
        let capture1 = regex_first_number.captures(&modified_line).unwrap();
        let capture2 = regex_last_number.captures(&modified_line).unwrap();
        let result = capture1[1].to_string()+&capture2[1];
        print!("{}:",result);
        sum += result.parse::<i32>().unwrap();
        print!("{}",sum);
        println!();
    }

    // print!("{}", sum);
    Ok(())
}
