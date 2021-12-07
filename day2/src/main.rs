use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
    let (mut dp,mut hp) = (0,0);
    let file = File::open("./day2.input").unwrap();
    let reader = BufReader::new(file); 
    for line in reader.lines(){
        let line = line.unwrap();
        let posval: Vec<&str> = line.split(" ").collect();
        match posval[0]{
            "forward" => hp+= posval[1].parse::<i64>().unwrap(),
            "down" => dp+= posval[1].parse::<i64>().unwrap(),
            _ => dp-= posval[1].parse::<i64>().unwrap()
        }
    }
    println!("{} * {}", dp,hp);
    println!("{}",dp*hp);
}

