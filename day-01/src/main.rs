use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = "./input.txt";
    match read_lines(path) {
        Ok(lines) => {
            let calibration_values: Vec<String> = lines
                .map(|line| line.unwrap_or_default().to_string())
                .collect();
            println!("{}", sum_calibration_values(calibration_values))
        }
        Err(e) => {
            println!("There was an error reading {}: {}", path, e.to_string())
        }
    }
}

fn sum_calibration_values(calibration_values: Vec<String>) -> i32 {
    let sum: i32 = calibration_values
        .iter()
        .map(|value| {
            let mut numbers: Vec<char> = vec![];
            // TODO: It's faster to iterate from start and end and stop if first and last were found
            for character in value.chars() {
                if character.is_numeric() {
                    numbers.push(character);
                }
            }

            let str_number = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap());
            return str_number.parse::<i32>().unwrap();
        })
        .sum();

    sum
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
