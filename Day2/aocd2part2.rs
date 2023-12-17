use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("C:\\Users\\vasil\\Desktop\\input.txt")?;
    let reader = BufReader::new(file);

    let regex_id = Regex::new(r"^Game\s(\d+):\s").unwrap();
    let mut result = 0;

    //read lines
    for line in reader.lines() {
        let mut blue_max = 0;
        let mut red_max = 0;
        let mut green_max = 0;
        let line_string = line.unwrap();
        let line_string_no_game = regex_id.replace(&line_string, "").to_string();

        let bag_reveal = line_string_no_game.split(";");
        //read draws from bag
        for draws in bag_reveal {
            let cube_split = draws.split(",");
            //read pairs
            for cube in cube_split {
                let mut iter = cube.split_whitespace();
                let cube_number = iter.next().unwrap().parse::<i32>().unwrap();
                let color = iter.next().unwrap();
                if color == "red" && cube_number > red_max {
                    red_max = cube_number;
                } 
                else if color == "green" && cube_number > green_max {
                    green_max = cube_number;
                } 
                else if color == "blue" && cube_number > blue_max {
                    blue_max = cube_number;
                }
            }
        }
        result += blue_max*red_max*green_max;
    }

    println!("{}",result);

    Ok(())
}