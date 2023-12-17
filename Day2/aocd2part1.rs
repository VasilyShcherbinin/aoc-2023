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
        let line_string = line.unwrap();
        let game_id = regex_id.captures(&line_string).unwrap(); //&game_id[1]
        let line_string_no_game = regex_id.replace(&line_string, "").to_string();
        let mut rc = 0;

        let bag_reveal = line_string_no_game.split(";");
        //read draws from bag
        for draws in bag_reveal {
            let cube_split = draws.split(",");
            //read pairs
            for cube in cube_split {
                let mut iter = cube.split_whitespace();
                let cubeNumber = iter.next().unwrap();
                let color = iter.next().unwrap();

                if color == "red" && cubeNumber.parse::<i32>().unwrap() > 12 {
                    rc = 1;
                    break;
                } 
                else if color == "green" && cubeNumber.parse::<i32>().unwrap() > 13 {
                    rc = 1;
                    break;
                } 
                else if color == "blue" && cubeNumber.parse::<i32>().unwrap() > 14 {
                    rc = 1;
                    break;
                }
            }
        }
        if rc == 0 {
            result += &game_id[1].parse::<i32>().unwrap(); 
        }
    }

    println!("{}",result);

    Ok(())
}
