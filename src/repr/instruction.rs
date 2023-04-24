use std::error::Error;
use std::fmt::Display;
use std::fmt;
use num_traits::Num;

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
    pub high: bool,
    pub low: bool,
    pub signed: bool,
    pub set_flags: bool,
    pub operand_a: Operand,
    pub operand_b: Operand
}

pub enum InstrType {
    Regular(u16),
    Long(u32)
}

impl Into<InstrType> for Instruction {
    /**
     * Takes a Sim6 instruction and converts it to its binary representation
     */
    fn into(self) -> InstrType {
        let opcode:u16 = self.opcode.into();
        let opcode = opcode << 10;

        let high = self.high as u16;
        let high:u16 = high << 9;

        let low = self.low as u16;
        let low:u16 = low << 8;

        let flag = self.set_flags as u16;
        let flag:u16 = flag << 7;

        let signed = self.signed as u16;
        let signed:u16 = signed << 6;

        let operand_b_code:u16 = self.operand_b.clone().into();
        let operand_a_code:u16 = self.operand_a.into();

        let upper_instr = 0 | opcode | high | low | flag | signed;

        match self.operand_b {
            Operand::Register(_) | Operand::ShortImmediate(_) => InstrType::Regular(upper_instr | operand_a_code << 3 | operand_b_code),
            Operand::LargeImmediate(_) => InstrType::Long(u32::from(upper_instr) << 16 | u32::from(operand_a_code) << 16 | operand_b_code as u32)
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
     * Creates an instruction from the given parameters, auto-calculates the high, low, flag and 
     * signed bits.
     */
    pub fn new(opcode:Opcode, operand_a:Operand, operand_b:Operand) -> Instruction {
        let high:bool;
        let low:bool;
        match &operand_a {
            Operand::Register(reg) => {
                high = reg.is_high_reg();
                low = reg.is_low_reg();
            },

            Operand::ShortImmediate(_) 
             | Operand::LargeImmediate(_) => panic!("Found immediate in 1st operand position")
        };

        Instruction {
            signed: opcode.is_signed(),
            set_flags: opcode.set_flags(),
            opcode: opcode,
            high: high,
            low: low,
            operand_a: operand_a,
            operand_b: operand_b
        }
    }
}


/**
 * Takes a string representing a number in decimal, hex, or binary, removes the "0x" or "0b" prefix if
 * necessary, and returns the value as type `T`. 
 * 
 * Will return a `FromStrRadixErr` if the number is invalid.
 */
fn convert_imm_str_to_unsigned<T: Num>(original:&str) -> Result<T, <T as Num>::FromStrRadixErr> {
    let immediate:T;
    if original.starts_with("0x") {
        immediate = T::from_str_radix(original.strip_prefix("0x").unwrap(), 16)?;
    } else if original.starts_with("0b") {
        immediate = T::from_str_radix(original.strip_prefix("0b").unwrap(), 2)?;
    } else {
        immediate = T::from_str_radix(original, 10)?;
    }

    Ok(immediate)
}


/**
 * Takes a string representing an integer either in decimal, hex (with the prefix '0x'), or binary (with
 * the prefix '0b') and returns an `Opcode::LongImmediate` or an `Opcode::ShortImmediate` depending on the
 * opcode provided.
 */
fn get_immediate_from_string(opcode:&Opcode, original:&str) -> Result<Operand, Box<dyn Error>> {
    let immediate = convert_imm_str_to_unsigned(original)?;
    match opcode {
        Opcode::MovI => Ok(Operand::LargeImmediate(immediate)),
        _ => Ok(Operand::ShortImmediate(immediate.try_into()?))
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data {
    pub bytes:Vec<u8>
}

impl From<&str> for Data {
    /**
     * Takes a string and converts it into a `Vec<u8>` for the `Data` struct.
     */
    fn from(line:&str) -> Data {
        let index = line.find(":").unwrap_or(0);
        let tokens:Vec<&str> = line[index..].split_whitespace().collect();

        // first token in the kind of data expected, byte, 2 byte word, 4 byte long word, array of bytes
        // or an ascii string with a null byte auto-appended.
        match *tokens.get(0).expect(&format!("Insufficient tokens in data line: '{}'", line)) {
            ".byte" => {
                Data {
                    bytes: vec![
                        convert_imm_str_to_unsigned(
                            tokens.get(1).expect(&format!("Insufficient tokens in data line: '{}'", line))
                        ).unwrap()
                    ]
                }
            },
            
            ".word" => {
                let immediate:u16 = convert_imm_str_to_unsigned(
                    tokens.get(1).expect(&format!("Insufficient tokens in data line: '{}'", line))
                ).unwrap();

                Data {
                    bytes: immediate.to_be_bytes().to_vec()
                }
            },

            ".long" => {
                let immediate:u32 = convert_imm_str_to_unsigned(
                    tokens.get(1).expect(&format!("Insufficient tokens in data line: '{}'", line))
                ).unwrap();

                Data {
                    bytes: immediate.to_be_bytes().to_vec()
                }
            },

            ".array" => {
                let bytes:Vec<u8> = tokens[1..].into_iter()
                                               .map(|b| convert_imm_str_to_unsigned(b).unwrap())
                                               .collect();
                Data {
                    bytes: bytes
                }
            },

            ".asciiz" => {
                let mut string = line[line.find("`").unwrap() + 1 .. line.len() - 1].as_bytes().to_vec();
                string.push(0x00);

                Data {
                    bytes: string
                }
            }

            datatype => panic!("'{}' is not a valid data instruction type", datatype)
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Data({:?})", self.bytes.clone().iter().map(|num| format!("0x{:02X?}", num)).collect::<Vec<String>>())
    }
}


#[derive(Debug, Clone)]
pub enum InstructionOrData {
    Instruction(Instruction),
    Data(Data)
}

impl Display for InstructionOrData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstructionOrData::Instruction(instr) => write!(f, "{:?}", instr),
            InstructionOrData::Data(data) => write!(f, "{}", data)
        }
    }
}

impl Into<Instruction> for InstructionOrData {
    fn into(self) -> Instruction {
        match self {
            InstructionOrData::Instruction(instr) => instr,
            InstructionOrData::Data(_) => panic!("{:?} is not an instruction", self)
        }
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
            InstrType::Regular(bin) => assert_eq!(bin, 0x07C1),
            _ => panic!("Invalid")
        }

        let binary:InstrType = Instruction::new(Opcode::Addc, Operand::Register(Register::Ax), Operand::Register(Register::None)).into();
        match binary {
            InstrType::Regular(bin) => assert_eq!(bin, 0x0F80),
            _ => panic!("Invalid")
        }

        let binary:InstrType = Instruction::new(Opcode::In, Operand::Register(Register::Dl), Operand::ShortImmediate(5)).into();
        match binary {
            InstrType::Regular(bin) => assert_eq!(bin, 0x4D1D),
            _ => panic!("Invalid")
        }

        let binary:InstrType = Instruction::new(Opcode::MovI, Operand::Register(Register::Sp), Operand::LargeImmediate(700)).into();
        match binary {
            InstrType::Long(bin) => assert_eq!(bin, 0x5B07_02BC),
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


    #[test]
    fn test_get_valid_data() {
        assert_eq!(Data::from(".byte 25"), Data { bytes: vec![25] });
        assert_eq!(Data::from(".byte 0x50"), Data { bytes: vec![0x50] });
        assert_eq!(Data::from(".word 0xAABB"), Data { bytes: vec![0xAA, 0xBB] });
        assert_eq!(Data::from(".word 0b1010101010101010"), Data { bytes: vec![0xAA, 0xAA] });
        assert_eq!(Data::from(".long 0x12345678"), Data { bytes: vec![0x12, 0x34, 0x56, 0x78] });
        assert_eq!(Data::from(".array 25 40 32 18"), Data { bytes: vec![25, 40, 32, 18] });
        assert_eq!(Data::from(".array 0xAC 40 0b11001100 18"), Data { bytes: vec![0xAC, 40, 0b11001100, 18] });
        assert_eq!(Data::from(".asciiz `Hey you!`"), Data { bytes: vec![0x48, 0x65, 0x79, 0x20, 0x79, 0x6F, 0x75, 0x21, 0x00] });
    }

    #[test]
    #[should_panic]
    fn test_invalid_data_type() {
        _ = Data::from(".bad 70");
    }

    #[test]
    #[should_panic]
    fn test_data_pos_overflow() {
        _ = Data::from(".long 7000000000");
    }

    #[test]
    #[should_panic]
    fn test_invalid_int_prefix() {
        _ = Data::from(".byte 0c55");
    }
}
