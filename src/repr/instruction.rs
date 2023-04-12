use super::register::Register;
use super::opcode::Opcode;


/**
 * Represents a Sim6 instruction
 */
#[derive(Debug)]
pub struct Instruction {
    opcode: Opcode,
    register_code: u16,
    dest_r: Register,
    target_r: Register
}

impl Into<u16> for Instruction {
    /**
     * Takes a Sim6 instruction and converts it to its binary representation
     */
    fn into(self) -> u16 {
        let opcode:u16 = self.opcode.into();
        let target:u16 = self.target_r.into();
        let dest:u16 = self.dest_r.into();

        let opcode:u16 = opcode << 10 | self.register_code << 6 | dest << 3 | target;

        opcode
    }
}

impl From<&str> for Instruction {
    /**
     * Takes a string representing a Sim6 instruction and converts it to an `Instruction`
     */
    fn from(line:&str) -> Instruction {
        let tokens:Vec<String> = line.split_whitespace().map(|token| token.replace(",", "").to_owned()).collect();
        let reg_a = Register::from(tokens.get(1).unwrap_or(&String::from("none")));
        let reg_b = Register::from(tokens.get(2).unwrap_or(&String::from("none")));

        Instruction {
            opcode: Opcode::from(tokens.get(0).unwrap()),
            register_code: Register::get_reg_code(&reg_a, &reg_b),
            dest_r: reg_a,
            target_r: reg_b
        }
    }
}

impl Instruction {
    /**
     * Creates an instruction from the given parameters, auto-calculates the register code
     */
    #[allow(dead_code)]
    fn new(opcode:Opcode, dest_r:Register, target_r:Register) -> Instruction {
        Instruction {
            opcode: opcode,
            register_code: Register::get_reg_code(&dest_r, &target_r),
            dest_r: dest_r,
            target_r: target_r
        }
    }
}



#[cfg(test)]
mod tests {
    use super::Instruction;
    use crate::repr::opcode::Opcode;
    use crate::repr::register::Register;


    #[test]
    fn test_gen_binary() {
        let mut instr_bin:u16 = Instruction::new(Opcode::Nop, Register::None, Register::None).into();
        assert_eq!(instr_bin, 0x0000);

        instr_bin = Instruction::new(Opcode::Add, Register::Ax, Register::Bx).into();
        assert_eq!(instr_bin, 0x07C1);

        instr_bin = Instruction::new(Opcode::Add, Register::Al, Register::Bh).into();
        assert_eq!(instr_bin, 0x0641);

        instr_bin = Instruction::new(Opcode::Addc, Register::Ax, Register::None).into();
        assert_eq!(instr_bin, 0x0B00);

        instr_bin = Instruction::new(Opcode::PushA, Register::None, Register::None).into();
        assert_eq!(instr_bin, 0x3000);
    }
}
