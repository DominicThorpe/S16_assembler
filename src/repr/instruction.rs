use super::register::Register;
use super::opcode::Opcode;


#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Register(Register),
    ShortImmediate(u8),
    LargeImmediate(u16)
}

impl Into<u16> for Operand {
    fn into(self) -> u16 {
        match self {
            Operand::Register(reg) => reg.into(),
            Operand::ShortImmediate(imm) => imm.into(),
            Operand::LargeImmediate(imm) => imm.into()
        }
    }
}


/**
 * Represents a Sim6 instruction
 */
#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
    register_code: u16,
    operand_a: Operand,
    operand_b: Operand
}

enum InstrType {
    Regular(u16),
    Long(u32)
}

impl Into<InstrType> for Instruction {
    /**
     * Takes a Sim6 instruction and converts it to its binary representation
     */
    fn into(self) -> InstrType {
        let opcode:u16 = self.opcode.into();
        let operand_a_code:u16 = self.operand_a.into();
        let operand_b_code:u16 = self.operand_b.clone().into();

        match self.operand_b {
            Operand::Register(_) => InstrType::Regular(opcode << 10 | self.register_code << 6 | operand_b_code << 3 | operand_a_code),
            Operand::ShortImmediate(_) => InstrType::Regular(opcode << 10 | self.register_code << 6 | operand_b_code << 3 | operand_a_code),
            Operand::LargeImmediate(_) => InstrType::Long((opcode as u32) << 26 | (self.register_code as u32) << 22 | (operand_a_code as u32) << 16 | operand_b_code as u32)
        }
    }
}

impl From<&str> for Instruction {
    /**
     * Takes a string representing a Sim6 instruction and converts it to an `Instruction`
     */
    fn from(line:&str) -> Instruction {
        let tokens:Vec<String> = line.split_whitespace().map(|token| token.replace(",", "").to_owned()).collect();

        let operand_a = Operand::Register(Register::from(tokens.get(1).unwrap_or(&String::from("none"))));
        match tokens.get(2).unwrap_or(&String::from("none")).parse::<u64>().is_ok() {
            false => { // is a register
                let operand_b = Operand::Register(Register::from(tokens.get(2).unwrap_or(&String::from("none"))));
                return Instruction::new(Opcode::from(tokens.get(0).unwrap()), operand_a, operand_b);
            },

            true  => { // is an immediate
                if Opcode::from(tokens.get(0).unwrap()) != Opcode::PushI {
                    let operand_b = Operand::ShortImmediate(tokens.get(2).unwrap().parse::<u8>().unwrap());
                    return Instruction::new(Opcode::from(tokens.get(0).unwrap()), operand_a, operand_b);
                } else {
                    let operand_b = Operand::LargeImmediate(tokens.get(2).unwrap().parse::<u16>().unwrap());
                    return Instruction::new(Opcode::from(tokens.get(0).unwrap()), operand_a, operand_b);
                }
            }
        }
    }
}

impl Instruction {
    /**
     * Creates an instruction from the given parameters, auto-calculates the register code
     */
    #[allow(dead_code)]
    fn new(opcode:Opcode, operand_a:Operand, operand_b:Operand) -> Instruction {
        Instruction {
            opcode: opcode,
            register_code: Register::get_reg_code(&operand_a, &operand_b),
            operand_a: operand_a,
            operand_b: operand_b
        }
    }
}



#[cfg(test)]
mod tests {
    use super::{Instruction, Operand, InstrType};
    use crate::repr::opcode::Opcode;
    use crate::repr::register::Register;


    #[test]
    fn test_gen_instrs() {
        assert_eq!(Instruction::from("Nop"), Instruction::new(Opcode::Nop, Operand::Register(Register::None), Operand::Register(Register::None)));
        assert_eq!(Instruction::from("ADD ax, bx"), Instruction::new(Opcode::Add, Operand::Register(Register::Ax), Operand::Register(Register::Bx)));
        assert_eq!(Instruction::from("ADDC ax"), Instruction::new(Opcode::Addc, Operand::Register(Register::Ax), Operand::Register(Register::None)));
        assert_eq!(Instruction::from("in dl, 5"), Instruction::new(Opcode::In, Operand::Register(Register::Dl), Operand::ShortImmediate(5)));
        assert_eq!(Instruction::from("pushi sp, 700"), Instruction::new(Opcode::PushI, Operand::Register(Register::Sp), Operand::LargeImmediate(700)));
    }


    #[test]
    fn test_gen_binary() {
        let binary:InstrType = Instruction::new(Opcode::Nop, Operand::Register(Register::None), Operand::Register(Register::None)).into();
        match binary {
            InstrType::Regular(bin) => assert_eq!(bin, 0x0000),
            _ => panic!("Invalid")
        }

        let binary:InstrType = Instruction::new(Opcode::Add, Operand::Register(Register::Ax), Operand::Register(Register::Bx)).into();
        match binary {
            InstrType::Regular(bin) => assert_eq!(bin, 0x07C8),
            _ => panic!("Invalid")
        }

        let binary:InstrType = Instruction::new(Opcode::Addc, Operand::Register(Register::Ax), Operand::Register(Register::None)).into();
        match binary {
            InstrType::Regular(bin) => assert_eq!(bin, 0x08C0),
            _ => panic!("Invalid")
        }

        let binary:InstrType = Instruction::new(Opcode::In, Operand::Register(Register::Dl), Operand::ShortImmediate(5)).into();
        match binary {
            InstrType::Regular(bin) => assert_eq!(bin, 0x462B),
            _ => panic!("Invalid")
        }

        let binary:InstrType = Instruction::new(Opcode::PushI, Operand::Register(Register::Sp), Operand::LargeImmediate(700)).into();
        match binary {
            InstrType::Long(bin) => assert_eq!(bin, 0x5307_02BC),
            _ => panic!("Invalid")
        }
    }
}
