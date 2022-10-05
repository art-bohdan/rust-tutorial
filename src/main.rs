#![allow(unused)]

mod restaurant;
use crate::restaurant::order_food;
use std::io::{self, stdin};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

 // Crates : Modules that produce a library or executable
    // Modules : Organize and handle privacy
    // Packages : Build, test and share crates
    // Paths : Away of naming an item such as a struct, function

    //Result has 2 varients Ok and Err
        // enum Result<T, E> {
        //  Ok(T),
        //  Err(E), }
        // Where T represents the data typeof the value returns and E
        // the type of Err
fn main() {
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file : {:?}",error);
        }
    };
    write!(output, "Just some\nRandom words").expect("Failed to write to file");
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("random.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file : {:?}", error)
            }, 
            _other_error => panic!("Problem opening file : {:?}", error),
        },
    };
}    
