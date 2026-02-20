use rand::Rng;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() -> io::Result<()> {
    // Count lines first
    let f = File::open("../manifesto.txt")?;
    let reader = BufReader::new(f);
    let file_length: u32 = reader.lines().count() as u32;
    
    // Generate random line number
    let mut rng = rand::thread_rng();
    let mut random_line: u32 = rng.gen_range(0..file_length);
    
    // Read to the random line
    let f = File::open("../manifesto.txt")?;
    let reader = BufReader::new(f);

    // Debug!  Show me the file length and the random line!
    // println!("File length: {}, Random line: {}", file_length, random_line);
    
    // ... read to line random_line
    let mut counter = 0;
    for line in reader.lines() {
        let unwrapped_line = line.unwrap();
        if counter == random_line {
            // if the line is empty, get the next one instead
            if unwrapped_line == "" {
                random_line += 1;
                counter += 1;
            } else {
            println!("{}", unwrapped_line);
            break;
            }
        } else {
            counter += 1;
        }
        // println!("Counter: {}, Random Line: {}, Line: {}", counter, random_line, unwrapped_line);
    }
    
    Ok(())
}