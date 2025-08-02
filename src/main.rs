use std::process;
use std::env;
use std::vec::Vec;
use std::path::Path;
use std::fs::File;
use std::io::{self};
use std::io::prelude::*;
use colored::Colorize;

// Called if program is run without CLI args
// Exits program and doesn't return anything
fn usage() {

    println!("Missing argument(s)");
    println!("Usage: grep 'pattern' 'filename'");
    process::exit(0x0100);
}

// Read entire file into a buffer
fn read_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
        let file = File::open(filename)?;
        return Ok(io::BufReader::new(file).lines());
}

// Truncates line until first space, but minimum 32 chars long
// @Return: truncated line (type &str)
fn truncate(line: &mut String) -> &str {

    let base_max_len = 32;
    
    let indices = line.char_indices().skip(base_max_len);
    for(index, chr) in indices {
        if chr == ' ' {
            *line = line[..index].to_string();
            break;
        }
    }
    return line;
}

// Parse buffer created by read_file() and add lines
// to result vector. 
// Truncates lines that are longer than line_max_len
// @Return: vector with lines truncated to <= line_max_len
fn parse_file(filename: &str) -> Vec<String> {

    let line_max_len = 32;
    let mut line_vector = Vec::new();

    if let Ok(lines) = read_file(filename){
        for mut line in lines.map_while(Result::ok){
            let trunc = if line.len() > line_max_len { 
                truncate(&mut line);
                line.to_string()
            } else {
                line
            };
            line_vector.push(trunc);
        }
    }
    return line_vector;
} 

// Iterates over lines vector and adds lines with matches
// to a new result vector.
// Also counts the number of 'pattern' occurences
// @Return: tuple3(lines with matches, line numbers, match count)
fn find_pattern(lines: Vec<String>, pattern: &str) -> 
(Vec<String>, Vec<i32>, i16) {

    let mut count: i16   = 0;
    let mut current_line = 1;
    let mut matches      = Vec::new();
    let mut line_numbers = Vec::new();

    for mut line in lines {
        if line.contains(pattern) {
            let _ = splice(&mut line, pattern);
            matches.push(line);
            line_numbers.push(current_line);
            count += 1;
        }
        current_line += 1;
    }
    (matches, line_numbers, count)
}

// Find pattern within a trunced string and recolor it
fn splice(line: &mut String, pattern: &str) -> std::io::Result<()> {

    let _ = line
        .split_whitespace()
        .find(|&word| word.trim_matches(
              |c: char| !c.is_alphanumeric()) == pattern);

    Ok(())
}


// Prints truncated lines that contain a match for 'pattern'
// @Return: result, always Ok(())
fn print_matches(lines: &Vec<String>, 
                 line_nums: &Vec<i32>,
                 count: i16,
                 pattern: &str
    ) -> std::io::Result<()> {
   
    let mut current_line: usize = 0;
    println!("Pattern found on {} line(s)", count);

    for line in lines {
        print!("{}: ", line_nums[current_line]);
        for word in line.split_whitespace() {
            if word == pattern {
                print!("{} ", word.green());
            } else {
                print!("{} ", word);
            }
        }
        println!();
        current_line += 1;
    }

    Ok(())
}

// Parse CLI arguments
// @Return: tuple(pattern, name of file)
fn parse_args(args: &[String]) -> (&str, &str){

    if args.len() < 3 {
        usage();
    }

    let pattern  = &args[1];
    let filename = &args[2];
    return (pattern, filename);
}

fn main() -> std::io::Result<()> {

    let args: Vec<String>   = env::args().collect(); 
    let (pattern, filename) = parse_args(&args);

    let lines = parse_file(filename);
    let (matches, lines, count) = find_pattern(lines, pattern); 
    let _ = print_matches(&matches, &lines, count, pattern);
    Ok(())
}
