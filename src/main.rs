use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write, Seek};
use std::env;

mod assembler;
mod repr;
mod validation;
mod label_table;

use assembler::process_line;
use label_table::get_label_table;
use repr::instruction::{InstrType, InstructionOrData};



#[allow(unused_variables)]
fn main() {
    let cmd_args:Vec<String> = env::args().collect();
    let filename:&str = cmd_args.get(1).expect("Expected <input file path>.asm <output file path>.sse");
    let output_name:&str = cmd_args.get(2).expect("Expected <input file path>.asm <output file path>.sse");

    if !filename.ends_with(".asm") {
        panic!("Input filename must end in .asm");
    }

    if !output_name.ends_with(".sse") {
        panic!("Output filename must end in .sse");
    }

    let mut input_file = OpenOptions::new().read(true).open(filename).unwrap();

    let label_table:HashMap<String, usize> = get_label_table(&input_file);
    input_file.rewind().unwrap();

    let mut data_mode = true;
    let input_lines = BufReader::new(&input_file).lines().filter_map(|line| match line.unwrap().trim() {
        "" => None, 
        l => process_line(l, &label_table, &mut data_mode)
    });

    let output_file = OpenOptions::new().create(true)
                                        .truncate(true)
                                        .write(true)
                                        .open(output_name)
                                        .unwrap();
    let mut writer = BufWriter::new(output_file);

    let mut bytes:Vec<u8> = vec![0x2E, 0x64, 0x61, 0x74, 0x61, 0x3A]; // ".data:" in ASCII
    let mut data_mode = true;
    for line in input_lines {
        match line {
            InstructionOrData::Data(data) => {
                bytes.append(&mut data.bytes.clone().as_mut_slice().to_vec());
            } 

            InstructionOrData::Instruction(instr) => {
                if data_mode {
                    data_mode = false;
                    bytes.append(&mut ".code:".as_bytes().to_vec()); // ".code:" in ASCII 
                }

                let instr_type:InstrType = instr.into();

                match instr_type {
                    InstrType::Regular(reg) => bytes.append(&mut reg.to_be_bytes().to_vec()),
                    InstrType::Long(long) => bytes.append(&mut long.to_be_bytes().to_vec())
                } 
            }
        }
    }

    writer.write_all(&bytes).unwrap();
}
