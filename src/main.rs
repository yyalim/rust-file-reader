use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn get_file(path: &str) -> File {
  let file = File::open(path);
  return match file {
      Ok(file) => file,
      Err(error) => {
          match error.kind() {
              std::io::ErrorKind::NotFound => {
                  panic!("File not found: {}", error)
              }
              _ => {
                  panic!("Error opening file: {}", error)
              }
          }
      }
  };
}

fn read_file(file: File) {
  let reader = BufReader::new(file);
  for line in reader.lines() {
      match line {
          Ok(line) => println!("{}", line),
          Err(error) => {
              panic!("Error reading line: {}", error)
          }
      }
  }
}

fn get_path(args: &Vec<String>) -> Option<&String> {
  match args.first() {
    Some(first_value) => Some(first_value),
    None => panic!("No file path provided")
  }
}

fn main() {
  let args = env::args().collect();
  let path = get_path(&args);

  if let Some(path) = path {
    let file = get_file(&path);
    read_file(file);
  }
}
