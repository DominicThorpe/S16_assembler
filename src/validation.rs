use crate::repr::instruction::{Instruction, Operand};
use crate::repr::{opcode::Opcode, register::Register};
use std::{fmt, error::Error};


#[derive(Debug, Clone)]
struct ValidationError(Instruction);

impl Error for ValidationError {}

impl fmt::Display for ValidationError {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} is not a valid instruction!", self.0)
    }
}


/**
 * Takes an instruction and validates the register code and the operand types and values
 */
pub fn validate_instruction(instr:&Instruction) -> Result<(), Box<dyn Error>> {
    match instr.opcode {
        // No operands
        Opcode::Nop | Opcode::PopA | Opcode::PushA | Opcode::PopF | Opcode::PushF | Opcode::Ret | Opcode::Ccry | Opcode::Scry 
         | Opcode::Eitr | Opcode::Ditr | Opcode::Iret => {
            // validate the register code
            if instr.register_code != 0 {
                return Err(Box::new(ValidationError(instr.clone())));
            }

            // validate operand a
            match &instr.operand_a {
                Operand::ShortImmediate(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError(instr.clone()))),
                Operand::Register(reg) => {
                    match reg {
                        Register::None => {},
                        _ => return Err(Box::new(ValidationError(instr.clone())))
                    }
                }
            }

            // validate operand b
            match &instr.operand_b {
                Operand::ShortImmediate(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError(instr.clone()))),
                Operand::Register(reg) => {
                    match reg {
                        Register::None => {},
                        _ => return Err(Box::new(ValidationError(instr.clone())))
                    }
                }
            }
        }

        // two register operands
        Opcode::Add | Opcode::Sub | Opcode::Cmp | Opcode::Move | Opcode::Swap | Opcode::Mul | Opcode::Imul | Opcode::Div 
         | Opcode::Idiv | Opcode::And | Opcode::Or | Opcode::Xor | Opcode::Sra | Opcode::Srl | Opcode::Sll | Opcode::Lda => {
            if !(instr.register_code == 0b1010 || instr.register_code == 0b0101 || instr.register_code == 0b1001 
                    || instr.register_code == 0b0110 || instr.register_code == 0b1111
                ) {
                    return Err(Box::new(ValidationError(instr.clone())));
            }

            match instr.operand_a {
                Operand::ShortImmediate(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError(instr.clone()))),
                _ => {}
            }

            match instr.operand_b {
                Operand::ShortImmediate(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError(instr.clone()))),
                _ => {}
            }
        }

        // one register operand
        Opcode::Addc | Opcode::Inc | Opcode::Subb | Opcode::Dec | Opcode::Neg | Opcode::Push | Opcode::Pop | Opcode::Csign 
         | Opcode::Not | Opcode::Clear | Opcode::Call | Opcode::Jump | Opcode::Jeq | Opcode::Jne | Opcode::Jgt | Opcode::Jle 
         | Opcode::Jgte | Opcode::Jlte | Opcode::Jzro | Opcode::Jnzro | Opcode::Jovf | Opcode::Jcry => {
            if !(instr.register_code != 0b1100 || instr.register_code != 0b0100 || instr.register_code != 0b1000 ) {
                return Err(Box::new(ValidationError(instr.clone())));
            }

            match &instr.operand_a {
                Operand::ShortImmediate(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError(instr.clone()))),
                Operand::Register(reg) => {
                    match reg {
                        Register::None => return Err(Box::new(ValidationError(instr.clone()))),
                        _ => {}
                    }
                }
            }

            match &instr.operand_b {
                Operand::ShortImmediate(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError(instr.clone()))),
                Operand::Register(reg) => {
                    match reg {
                        Register::None => {},
                        _ => return Err(Box::new(ValidationError(instr.clone())))
                    }
                }
            }
        }

        // one register and one 5-bit immediate
        Opcode::In | Opcode::Out | Opcode::Intr | Opcode::Into => {
            if !(instr.register_code != 0b1100 || instr.register_code != 0b0100 || instr.register_code != 0b1000 ) {
                return Err(Box::new(ValidationError(instr.clone())));
            }

            match instr.operand_a {
                Operand::ShortImmediate(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError(instr.clone()))),
                _ => {}
            }

            match instr.operand_b {
                Operand::Register(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError(instr.clone()))),
                Operand::ShortImmediate(imm) => {
                    if imm > 0x001F {
                        return Err(Box::new(ValidationError(instr.clone())))
                    }
                }
            }
        }

        // one register and one 16 bit immediate
        Opcode::MovI => {
            if !(instr.register_code != 0b1100 || instr.register_code != 0b0100 || instr.register_code != 0b1000 ) {
                return Err(Box::new(ValidationError(instr.clone())));
            }

            match instr.operand_a {
                Operand::ShortImmediate(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError(instr.clone()))),
                _ => {}
            }

            // large immediate cannot be out of range due to u16 type limits
            match instr.operand_b {
                Operand::Register(_) | Operand::ShortImmediate(_) => return Err(Box::new(ValidationError(instr.clone()))),
                _ => {}
            }
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::assembler::process_line;


    #[test]
    fn test_valid_nn_instrs() {
        process_line("  NOP");
        process_line("my_label: POPA");
        process_line("pusha");
        process_line("ret");
        process_line("scry");
        process_line("CcRy");
        process_line("__hello:      Eitr    ");
        process_line("Ditr");
        process_line("Iret");
    }


    #[test]
    fn test_valid_rn_instrs() {
        process_line("ADDC  ax");
        process_line("inc bl");
        process_line("Subb bh");
        process_line("Dec    dx");
        process_line("label:  Neg DX");
        process_line("_l_a_b_e_l: Push  aH");
        process_line("Pop Ah");
        process_line("Csign        ah");
        process_line("CLEAR rp");
    }


    #[test]
    fn test_valid_ri_instrs() {
        process_line("  in rp, 10");
        process_line("out ax 10");
        process_line("InTr rp, 0");
        process_line("lbl: Into, sp,,, 0");
    }

    #[test]
    fn test_valid_rl_instrs() {
        process_line("mOvi ax   700");
        process_line("mOvi ax   0");
    }


    #[test]
    fn test_valid_rr_instrs() {
        process_line("ADD ax bx");
        process_line("sub ax bx");
        process_line("move ah bh");
        process_line("And al bh");
        process_line("SRa al bl");
    }

    #[test]
    #[should_panic]
    fn test_nn_with_reg() {
        process_line("nop ax").unwrap();
    }


    #[test]
    #[should_panic]
    fn test_rr_with_one_reg() {
        process_line("add ax").unwrap();
    }


    #[test]
    #[should_panic]
    fn test_rr_with_imm() {
        process_line("add ax 10").unwrap();
    }


    #[test]
    #[should_panic]
    fn test_rn_with_two_reg() {
        process_line("addc ax sp").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_rn_with_imm() {
        process_line("addc 5").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_ri_with_no_imm() {
        process_line("out ax").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_ri_with_two_reg() {
        process_line("in ax sp").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_rl_with_two_reg() {
        process_line("movi ax sp").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_rl_with_no_reg() {
        process_line("addc 1000").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_short_operand_overflow() {
        process_line("in ax 32").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_long_operand_overflow() {
        process_line("movi ax 65536").unwrap();
    }
}
