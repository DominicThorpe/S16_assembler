use std::error::Error;

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
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub register_code: u16,
    pub operand_a: Operand,
    pub operand_b: Operand
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
     * Takes a string representing a Sim6 instruction and converts it to an `Instruction`, will panic if it
     * find an immediate too big for the number of bits given.
     */
    fn from(line:&str) -> Instruction {        
        let tokens:Vec<String> = line.split_whitespace().map(|token| token.replace(",", "").to_owned()).collect();

        let opcode = Opcode::from(tokens.get(0).unwrap());
        let operand_a = Operand::Register(Register::from(tokens.get(1).unwrap_or(&String::from("none"))));

        // get register operand or an immediate operand if the 1st character is a base-10 digit (hex and binary immediates
        // start with a prefix starting with 0)
        match tokens.get(2).unwrap_or(&String::from("none")).chars().nth(0).unwrap().is_digit(10) {
            false => { // is a register
                let operand_b = Operand::Register(Register::from(tokens.get(2).unwrap_or(&String::from("none"))));
                return Instruction::new(opcode, operand_a, operand_b);
            },

            true => {
                let operand_b = get_immediate_from_string(&opcode, tokens.get(2).unwrap()).unwrap();
                return Instruction::new(opcode, operand_a, operand_b)
            }
        }
    }
}

impl Instruction {
    /**
     * Creates an instruction from the given parameters, auto-calculates the register code
     */
    #[allow(dead_code)]
    pub fn new(opcode:Opcode, operand_a:Operand, operand_b:Operand) -> Instruction {
        Instruction {
            opcode: opcode,
            register_code: Register::get_reg_code(&operand_a, &operand_b),
            operand_a: operand_a,
            operand_b: operand_b
        }
    }
}


/**
 * Takes a string representing an integer either in decimal, hex (with the prefix '0x'), or binary (with
 * the prefix '0b') and returns an `Opcode::LongImmediate` or an `Opcode::ShortImmediate` depending on the
 * opcode provided.
 */
fn get_immediate_from_string(opcode:&Opcode, original:&str) -> Result<Operand, Box<dyn Error>> {
    let immediate:u16;
    if original.starts_with("0x") {
        immediate = u16::from_str_radix(&original[2..], 16)?;
    } else if original.starts_with("0b") {
        immediate = u16::from_str_radix(&original[2..], 2)?;
    } else {
        immediate = original.parse().unwrap();
    }

    match opcode {
        Opcode::MovI => Ok(Operand::LargeImmediate(immediate)),
        _ => Ok(Operand::ShortImmediate(immediate.try_into()?))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::repr::opcode::Opcode;
    use crate::repr::register::Register;


    #[test]
    fn test_gen_instrs() {
        assert_eq!(Instruction::from("Nop"), Instruction::new(Opcode::Nop, Operand::Register(Register::None), Operand::Register(Register::None)));
        assert_eq!(Instruction::from("ADD ax, bx"), Instruction::new(Opcode::Add, Operand::Register(Register::Ax), Operand::Register(Register::Bx)));
        assert_eq!(Instruction::from("ADDC ax"), Instruction::new(Opcode::Addc, Operand::Register(Register::Ax), Operand::Register(Register::None)));
        assert_eq!(Instruction::from("in dl, 5"), Instruction::new(Opcode::In, Operand::Register(Register::Dl), Operand::ShortImmediate(5)));
        assert_eq!(Instruction::from("movi sp, 700"), Instruction::new(Opcode::MovI, Operand::Register(Register::Sp), Operand::LargeImmediate(700)));
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

        let binary:InstrType = Instruction::new(Opcode::MovI, Operand::Register(Register::Sp), Operand::LargeImmediate(700)).into();
        match binary {
            InstrType::Long(bin) => assert_eq!(bin, 0x5307_02BC),
            _ => panic!("Invalid")
        }
    }


    #[test]
    fn test_get_immediate() {
        assert_eq!(get_immediate_from_string(&Opcode::Add, "0").unwrap(), Operand::ShortImmediate(0));
        assert_eq!(get_immediate_from_string(&Opcode::Add, "20").unwrap(), Operand::ShortImmediate(20));
        assert_eq!(get_immediate_from_string(&Opcode::Add, "31").unwrap(), Operand::ShortImmediate(31));
        assert_eq!(get_immediate_from_string(&Opcode::MovI, "65535").unwrap(), Operand::LargeImmediate(0xFFFF));

        assert_eq!(get_immediate_from_string(&Opcode::Add, "0b0").unwrap(), Operand::ShortImmediate(0));
        assert_eq!(get_immediate_from_string(&Opcode::Add, "0b11001").unwrap(), Operand::ShortImmediate(25));
        assert_eq!(get_immediate_from_string(&Opcode::Add, "0b11111").unwrap(), Operand::ShortImmediate(31));
        assert_eq!(get_immediate_from_string(&Opcode::MovI, "0b1111111111111111").unwrap(), Operand::LargeImmediate(0xFFFF));

        assert_eq!(get_immediate_from_string(&Opcode::Add, "0x000").unwrap(), Operand::ShortImmediate(0));
        assert_eq!(get_immediate_from_string(&Opcode::Add, "0x19").unwrap(), Operand::ShortImmediate(25));
        assert_eq!(get_immediate_from_string(&Opcode::Add, "0x1F").unwrap(), Operand::ShortImmediate(31));
        assert_eq!(get_immediate_from_string(&Opcode::MovI, "0xFFFF").unwrap(), Operand::LargeImmediate(0xFFFF));
    }
}
