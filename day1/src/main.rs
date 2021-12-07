use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let mut dc = 0;
    if let Ok(lines) = read_lines("./day1.input") {
        // Consumes the iterator, returns an (ptional) String
        let mut prev = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip.parse::<i32>().unwrap() > prev && prev != 0{
                    dc+=1;   
                }
                prev = ip.parse::<i32>().unwrap();
            }
        }
    }
    println!("{}",dc);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

