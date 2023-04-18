use std::collections::HashMap;

use crate::repr::instruction::*;
use crate::validation::*;


/**
 * Takes a line of S6 assembly and removes the label. Returns `None` if the line is just a label, otherwise
 * generates an `Instruction` for the line.
 */
pub fn process_line(line:&str, label_table:&HashMap<String, usize>, data_mode:&mut bool) -> Option<InstructionOrData> {  
    // this is a single-threaded assembler, therefore mutable static variable is ok
    if line == "code:" {
        *data_mode = false;
    }

    // get the line excluding any labels ending in ":"
    let mut line = match line.find(":") {
        None => line,
        Some(index) => (line[index + 1..]).trim()
    };

    // if the line was just a label, return `None`
    if line.is_empty() {
        return None;
    }

    // substitute a label for an absolute value
    let new_line;
    if let Some(index) = line.find("@")  {
        let label = line[index + 1..].to_owned();
        validate_label(&label).unwrap();

        new_line = line.replace(&format!("@{}", label), &label_table[&label].to_string());
        line = new_line.as_str();
    }

    match data_mode {
        true => {
            let data = Data::from(line);
            return Some(InstructionOrData::Data(data));
        }

        false => {
            let instr = Instruction::from(line);
            validate_instruction(&instr).unwrap();
            return Some(InstructionOrData::Instruction(instr));
        }
    }    
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::OpenOptions;
    use std::io::{BufRead, BufReader, Seek};

    use crate::label_table::get_label_table;
    use crate::repr::instruction::{Instruction, InstructionOrData};
    use crate::repr::opcode::Opcode;
    use crate::repr::instruction::Operand;
    use crate::repr::register::Register;
    use super::process_line;


    fn load_input_lines(filename:&str) -> Vec<InstructionOrData> {
        let mut input_file = OpenOptions::new().read(true)
                                               .open(filename)
                                               .unwrap();
        
        let label_table:HashMap<String, usize> = get_label_table(&input_file);
        input_file.rewind().unwrap();

        let mut data_mode = true;
        BufReader::new(&input_file).lines().filter_map(|line| match line.unwrap().trim() {
            "" => None, 
            l => process_line(l, &label_table, &mut data_mode)
        }).collect()
    }


    #[test]
    fn check_label_substitution() {
        let input_lines = load_input_lines("test_files/test_label_substitution.asm");
        assert_eq!(Instruction::new(Opcode::MovI, Operand::Register(Register::Cx), Operand::LargeImmediate(17)), input_lines[5].clone().into());
        assert_eq!(Instruction::new(Opcode::MovI, Operand::Register(Register::Ax), Operand::LargeImmediate(9)), input_lines[7].clone().into());
        assert_eq!(Instruction::new(Opcode::MovI, Operand::Register(Register::Bx), Operand::LargeImmediate(4)), input_lines[8].clone().into());
    }


    #[test]
    #[should_panic]
    fn test_mixed_code_data() {
        let _ = load_input_lines("test_files/test_mixed_code_data.asm");
    }
}
