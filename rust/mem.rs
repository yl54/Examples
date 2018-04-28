/* 
    This suite of introductory programs will focus
      on memory management operations.
*/

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("test_file_io()");
    test_file_io();
    println!();
}

/*
    This set of functions will focus on reading a file from
      disk, reading the words, and printing to another file.
*/
fn test_file_io() {
    // Initialize a path to the file we want to read from.
    let in_path = Path::new("input/input.txt");
    let display = in_path.display();

    // Check if the file exists.
    let mut in_file = match File::open(&in_path) {
        Err(why) => panic!("Error: Couldn't read {}: {}", display, why.description()),
        Ok(in_file) => in_file,
    };

    // Read from the file.
    let mut s = String::new();

    // Note: If there is a void function, then match to an error or ok.
    match in_file.read_to_string(&mut s) {
        Err(why) => panic!("Error: Couldn't save {} to string: {}", display, why.description()),
        Ok(_) => println!("{} contains:\n{}", display, s),
    };

    // Make a new file.
    let out_path = Path::new("output/output.txt");
    let out_display = out_path.display();
    let mut out_file = match File::create(&out_path) {
        Err(why) => panic!("Error: Couldn't create {}: {}", out_display, why.description()),
        Ok(out_file) => out_file,
    };

    // Write to another file.
    match out_file.write_all(s.as_bytes()) {
        Err(why) => panic!("Error: Couldn't write to {}: {}", out_display, why.description()),
        Ok(_) => println!("Successfully wrote to {}", out_display),
    };

    // Don't need to close the file stream once out fo scope.
    println!("Success!");
}

