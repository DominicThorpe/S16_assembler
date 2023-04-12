use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

mod repr;


#[allow(unused_variables)]
fn main() {
    let filename:&str = "test.asm";
    let output_name:&str = "output.exe";

    let input_file = OpenOptions::new().read(true).open(filename).unwrap();
    let input_lines = BufReader::new(input_file).lines().filter_map(|line| match line.unwrap().trim() {
        "" => None, 
        l => Some(repr::instruction::Instruction::from(l))
    });

    let output_file = OpenOptions::new().create(true).truncate(true).write(true).open(output_name);
    for line in input_lines {
        println!("{:?}", line);
    }
}
