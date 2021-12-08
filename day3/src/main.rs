use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
    let mut bits = vec![];
    let file = File::open("./day3.input").unwrap();
    let reader = BufReader::new(file); 
    for line in reader.lines(){
        let line = line.unwrap();
        bits.push(line);
    }
    
    let mut cnt = 0;
    let mut zc = 0;
    let mut oc = 0;
    let mut binstr:String = "".to_owned();
    while cnt < bits[0].len(){
        for i in 0..bits.len(){
            if bits[i].as_bytes()[cnt] as char == '0'{ zc+=1;
            }
            else{
                oc+=1;
            }
        }
        binstr.push_str(if oc >= zc {"1"} else {"0"});
        println!("0 count : {}",zc);
        println!("1 count : {}",oc);
        println!("-------------");
        cnt+=1;
        zc=0;
        oc=0;
    }
    let gammarate = isize::from_str_radix(&binstr, 2).unwrap();
    let max = isize::from_str_radix(&"1".repeat(cnt),2).unwrap(); //dynamic way to find 4095
    
    println!("{}",gammarate * (max-gammarate));
}


