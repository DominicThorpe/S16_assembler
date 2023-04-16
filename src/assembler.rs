use std::collections::HashMap;

use crate::repr::instruction::Instruction;
use crate::validation::{validate_instruction, validate_label};


/**
 * Takes a line of S6 assembly and removes the label. Returns `None` if the line is just a label, otherwise
 * generates an `Instruction` for the line.
 */
pub fn process_line(line:&str, label_table:&HashMap<String, usize>) -> Option<Instruction> {
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

    let instr = Instruction::from(line);
    validate_instruction(&instr).unwrap();
    
    Some(instr)
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::OpenOptions;
    use std::io::{BufRead, BufReader, Seek};

    use crate::label_table::get_label_table;
    use crate::repr::instruction::Instruction;
    use crate::repr::opcode::Opcode;
    use crate::repr::instruction::Operand;
    use crate::repr::register::Register;
    use super::process_line;


    #[test]
    fn check_label_substitution() {
        let mut input_file = OpenOptions::new().read(true).open("test_files/test_label_substitution.asm").unwrap();
        
        let label_table:HashMap<String, usize> = get_label_table(&input_file);
        println!("{:#?}", label_table);
        input_file.rewind().unwrap();

        let input_lines:Vec<Instruction> = BufReader::new(&input_file).lines().filter_map(|line| match line.unwrap().trim() {
            "" => None, 
            l => process_line(l, &label_table)
        }).collect();

        assert_eq!(input_lines[3], Instruction::new(Opcode::MovI, Operand::Register(Register::Cx), Operand::LargeImmediate(12)));
        assert_eq!(input_lines[5], Instruction::new(Opcode::MovI, Operand::Register(Register::Ax), Operand::LargeImmediate(4)));
    }
}
