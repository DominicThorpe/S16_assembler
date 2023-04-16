use std::collections::HashMap;

use crate::repr::instruction::Instruction;
use crate::validation::{validate_instruction, validate_label};


/**
 * Takes a line of S6 assembly and removes the label. Returns `None` if the line is just a label, otherwise
 * generates an `Instruction` for the line.
 */
pub fn process_line(line:&str, label_table:&HashMap<String, usize>) -> Option<Instruction> {
    let line = match line.find(":") {
        None => line,
        Some(index) => {
            validate_label(&line[..index]).unwrap();
            (line[index + 1..]).trim()
        }
    };

    let instr = Instruction::from(line);
    validate_instruction(&instr).unwrap();
    
    Some(instr)
}
