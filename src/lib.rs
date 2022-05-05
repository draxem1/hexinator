//Converts text files into hexadecimal
//Decodes hexadecimal files into text
#![feature(string_remove_matches)]

mod decode;
use std::io::BufRead;
use std::env::args;
use std::process;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::{BufReader,Write};
use decode::conversion::*;

#[derive(Debug)]
pub enum Crypt{
    Hex,
    Text,
    None,
}

pub fn get_args() -> (Crypt, String){

    let args: Vec<String> = args().collect();
    let mut file = String::new();
    let mut command = String::new();

    if args.len() < 3 {
        help_menu();
    }

    if args[1].contains("--code") || args[1].contains("--decode"){
        command = args[1].clone();

    } else {
        println!("Command or syntax not valid!!");
        help_menu();
    }

    if !Path::new(&args[2]).exists(){
        println!("Path or file doesn't exist!!");
        help_menu();

    } else {
        file = args[2].clone();
    }

    let crypt = match command.as_str() {
        "--code" => Crypt::Hex,
        "--decode"=> Crypt::Text,
        _ => Crypt::None,
    };

    (crypt, file)
}

fn help_menu(){
    let example = "\nExample: <COMMAND> <FILE>\n\n";
    let hex = "--code     [hexidecimal]\n";
    let text = "--decode   [text]\n";
    let file = "[path/filename]";

    let string = format!("{} <COMMAND> \t{} \t\t{} \n <FILE> \t{}",
        example, hex, text, file);

    println!("{}",string);

    process::exit(0);
}

fn read_file(file: &str) -> Vec<String> {
    let error_message = format!("failed to open {}", file);
    let file_to_read = File::open(file).expect(&error_message);
    let buf_reader = BufReader::new(file_to_read);
    let mut contents = Vec::new();
    
    for line in buf_reader.lines() {
        contents.push(line.expect("Failed to read Line"));
    }

    contents
}

//Converts file to hex
pub fn hexidecimal_encription(file: &str){
    let contents = read_file(file);
    let new_file_name = format!("{}_hex.txt", &file[0..(file.len()-4)]);
    let mut line = String::new();

    File::create(&new_file_name).expect("unable to create new file");

    let mut hex_file = fs::OpenOptions::new()
      .write(true)
      .append(true)
      .open(new_file_name)
      .unwrap();

    for i in contents {
        line.clear();

        for j in i.chars() {
            let decimal: u8 = j as u8;
            let hexidecimal = format!(" x{:X}", decimal);
            line.push_str(&hexidecimal);

        }
        write!(hex_file, "{}\n",line).expect("Couldn't append to file!!");
    }
}

//Converts hex in file to text
pub fn hex_to_text(file: &str){
    let contents = read_file(file);
    let new_file_name = format!("{}test.txt", &file[0..(file.len()-8)]);

    File::create(&new_file_name).expect("unable to create new file!");

    let mut text_file = fs::OpenOptions::new()
      .write(true)
      .append(true)
      .open(new_file_name)
      .unwrap();

      for i in contents{
            write!(text_file, "{}", inpairs(i)).expect("Couldn't append to file");
      }
}