use crate::repr::instruction::Instruction;
use crate::validation::{validate_instruction, validate_label};


/**
 * Takes a line of S6 assembly and removes the label. Returns `None` if the line is just a label, otherwise
 * generates an `Instruction` for the line.
 */
pub fn process_line(line:&str) -> Option<Instruction> {
    if line.ends_with(":") {
        validate_label(&line[..line.len() - 1]).unwrap()
        // label table processing here
    } 
    
    else {
        let line = match line.find(":") {
            None => line,
            Some(index) => {
                validate_label(&line[..index]).unwrap();
                (line[index + 1..]).trim()
            }
        };

        let instr = Instruction::from(line);
        validate_instruction(&instr).unwrap();
        return Some(instr)
    }

    None
}
