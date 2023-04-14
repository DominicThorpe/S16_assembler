use crate::repr::instruction::Instruction;
use crate::validation::validate_instruction;


/**
 * Takes a line of S6 assembly and removes the label. Returns `None` if the line is just a label, otherwise
 * generates an `Instruction` for the line.
 */
pub fn process_line(line:&str) -> Option<Instruction> {
    if line.ends_with(":") {
        // label table processing here
    } else {
        let line = match line.find(":") {
            Some(index) => (line[index + 1..]).trim(),
            None => line
        };

        let instr = Instruction::from(line);
        validate_instruction(&instr).unwrap();
        return Some(instr)
    }

    None
}
