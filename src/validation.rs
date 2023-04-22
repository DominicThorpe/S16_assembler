use crate::repr::instruction::{Instruction, Operand};
use crate::repr::{opcode::Opcode, register::Register};
use std::{fmt, error::Error};


#[derive(Debug, Clone)]
enum ValidationError {
    InvalidRegisterCodeError(u16, Opcode),
    RegisterNotNoneError(Register),
    MixedRegisterTypesError(Register, Register),
    RegisterIsNoneError(Register),
    OperandNotRegisterError(Operand),
    OperandNotShortImmediateError(Operand),
    OperandNotLongImmediateError(Operand),
    ImmediateTooLargeError(u16),
    LabelInvalidFormat(String)
}

impl Error for ValidationError {}

impl fmt::Display for ValidationError {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::InvalidRegisterCodeError(code, opcode) => write!(f, "{:04b} is not a valid register code for opcode {:?}", code, opcode),
            ValidationError::RegisterNotNoneError(reg) => write!(f, "Register {:?} should be None", reg),
            ValidationError::MixedRegisterTypesError(reg_a, reg_b) => write!(f, "Register {:?} and {:?} are either of different sizes or mixed high/low", reg_a, reg_b),
            ValidationError::RegisterIsNoneError(reg) => write!(f, "Register {:?} must not be None", reg),
            ValidationError::OperandNotRegisterError(operand) => write!(f, "Operand {:?} should be a register", operand),
            ValidationError::OperandNotShortImmediateError(operand) => write!(f, "Operand {:?} should be a short immediate", operand),
            ValidationError::OperandNotLongImmediateError(operand) => write!(f, "Operand {:?} should be a long immediate", operand),
            ValidationError::ImmediateTooLargeError(imm) => write!(f, "Immediate {} is too large", imm),
            ValidationError::LabelInvalidFormat(label) => write!(f, "Label '{:?}' is in an invalid format", label)
        }
    }
}


/**
 * Takes a label and validates that it is longer than 1 character contains only ascii alphanumeric characters and 
 * starts with a letter or an underscore.
 */
pub fn validate_label(label:&str) -> Result<(), Box<dyn Error>> {
    if !(label.chars().nth(0).unwrap().is_ascii_alphabetic() || label.chars().nth(0).unwrap() == '_') {
        return Err(Box::new(ValidationError::LabelInvalidFormat(label.to_string())));
    }

    if !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(Box::new(ValidationError::LabelInvalidFormat(label.to_string())));
    }

    Ok(())
}


/**
 * Takes a pair of `Operand`s which should represent `Operand::Register`s and returns `Ok(())` if they
 * match or a `ValidationError` if they are either of mixed sizes (16 and 8 bits) or if a high register
 * is paired with a low register.
 */
fn validate_register_operand_pair(operand_a:&Operand, operand_b:&Operand) -> Result<(), Box<dyn Error>> {
    let reg_a = match operand_a {
        Operand::Register(reg_a) => reg_a,
        _ => return Err(Box::new(ValidationError::OperandNotRegisterError(operand_a.clone())))
    };

    let reg_b = match operand_b {
        Operand::Register(reg_b) => reg_b,
        _ => return Err(Box::new(ValidationError::OperandNotRegisterError(operand_b.clone())))
    };

    match reg_a {
        Register::Ax | Register::Bx | Register::Cx | Register::Dx | Register::Sp | Register::Fp
         | Register::Bp | Register::Rp => {
            match reg_b {
                Register::Ax | Register::Bx | Register::Cx | Register::Dx | Register::Sp | Register::Fp
                 | Register::Bp | Register::Rp => return Ok(()),
                _ => return Err(Box::new(ValidationError::MixedRegisterTypesError(reg_a.clone(), reg_b.clone()))),
            }
         }
        
        Register::Ah | Register::Bh | Register::Ch | Register::Dh => {
            match reg_b {
                Register::Ah | Register::Bh | Register::Ch | Register::Dh => return Ok(()),
                _ => return Err(Box::new(ValidationError::MixedRegisterTypesError(reg_a.clone(), reg_b.clone()))),
            }
        }

        Register::Al | Register::Bl | Register::Cl | Register::Dl => {
            match reg_b {
                Register::Al | Register::Bl | Register::Cl | Register::Dl => return Ok(()),
                _ => return Err(Box::new(ValidationError::MixedRegisterTypesError(reg_a.clone(), reg_b.clone()))),
            }
        }

        Register::None | Register::Pc | Register::St => return Err(Box::new(ValidationError::RegisterIsNoneError(reg_a.clone())))
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
                return Err(Box::new(ValidationError::InvalidRegisterCodeError(instr.register_code, instr.opcode.clone())));
            }

            // validate operand a
            match &instr.operand_a {
                Operand::ShortImmediate(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError::OperandNotRegisterError(instr.operand_a.clone()))),
                Operand::Register(reg) => {
                    match reg {
                        Register::None => {},
                        _ => return Err(Box::new(ValidationError::RegisterNotNoneError(reg.clone())))
                    }
                }
            }

            // validate operand b
            match &instr.operand_b {
                Operand::ShortImmediate(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError::OperandNotRegisterError(instr.operand_a.clone()))),
                Operand::Register(reg) => {
                    match reg {
                        Register::None => {},
                        _ => return Err(Box::new(ValidationError::RegisterNotNoneError(reg.clone())))
                    }
                }
            }
        }

        // two register operands
        Opcode::Add | Opcode::Sub | Opcode::Cmp | Opcode::Move | Opcode::Swap | Opcode::Mul | Opcode::Mulu 
         | Opcode::Div | Opcode::Divu | Opcode::And | Opcode::Or | Opcode::Xor | Opcode::Sra | Opcode::Srl 
         | Opcode::Sll | Opcode::Lda | Opcode::Load | Opcode::Store | Opcode::Addu | Opcode::Subu => {
            if !(instr.register_code == 0b1010 || instr.register_code == 0b0101 || instr.register_code == 0b1001 
                    || instr.register_code == 0b0110 || instr.register_code == 0b1111
                ) {
                    return Err(Box::new(ValidationError::InvalidRegisterCodeError(instr.register_code, instr.opcode.clone())));
            }

            match instr.operand_a {
                Operand::ShortImmediate(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError::OperandNotRegisterError(instr.operand_a.clone()))),
                _ => {}
            }

            match instr.operand_b {
                Operand::ShortImmediate(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError::OperandNotRegisterError(instr.operand_a.clone()))),
                _ => {}
            }

            validate_register_operand_pair(&instr.operand_a, &instr.operand_b)?;
        }

        // one register operand
        Opcode::Addc | Opcode::Inc | Opcode::Subb | Opcode::Dec | Opcode::Neg | Opcode::Push | Opcode::Pop | Opcode::Csign 
         | Opcode::Not | Opcode::Clear | Opcode::Call | Opcode::Jump | Opcode::Jeq | Opcode::Jne | Opcode::Jgt | Opcode::Jle 
         | Opcode::Jgte | Opcode::Jlte | Opcode::Jzro | Opcode::Jnzro | Opcode::Jovf | Opcode::Jcry => {
            if !(instr.register_code != 0b1100 || instr.register_code != 0b0100 || instr.register_code != 0b1000 ) {
                return Err(Box::new(ValidationError::InvalidRegisterCodeError(instr.register_code, instr.opcode.clone())));
            }

            match &instr.operand_a {
                Operand::ShortImmediate(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError::OperandNotRegisterError(instr.operand_a.clone()))),
                Operand::Register(reg) => {
                    match reg {
                        Register::None => return Err(Box::new(ValidationError::RegisterIsNoneError(reg.clone()))),
                        _ => {}
                    }
                }
            }

            match &instr.operand_b {
                Operand::ShortImmediate(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError::OperandNotRegisterError(instr.operand_a.clone()))),
                Operand::Register(reg) => {
                    match reg {
                        Register::None => {},
                        _ => return Err(Box::new(ValidationError::RegisterNotNoneError(reg.clone())))
                    }
                }
            }
        }

        // one register and one 5-bit immediate
        Opcode::In | Opcode::Out | Opcode::Intr | Opcode::Into => {
            if !(instr.register_code != 0b1100 || instr.register_code != 0b0100 || instr.register_code != 0b1000 ) {
                return Err(Box::new(ValidationError::InvalidRegisterCodeError(instr.register_code, instr.opcode.clone())));
            }

            match instr.operand_a {
                Operand::ShortImmediate(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError::OperandNotRegisterError(instr.operand_a.clone()))),
                _ => {}
            }

            match instr.operand_b {
                Operand::Register(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError::OperandNotShortImmediateError(instr.operand_b.clone()))),
                Operand::ShortImmediate(imm) => {
                    if imm > 0x001F {
                        return Err(Box::new(ValidationError::ImmediateTooLargeError(imm as u16)))
                    }
                }
            }
        }

        // one register and one 16 bit immediate
        Opcode::MovI => {
            if !(instr.register_code != 0b1100 || instr.register_code != 0b0100 || instr.register_code != 0b1000 ) {
                return Err(Box::new(ValidationError::InvalidRegisterCodeError(instr.register_code, instr.opcode.clone())));
            }

            match instr.operand_a {
                Operand::ShortImmediate(_) | Operand::LargeImmediate(_) => return Err(Box::new(ValidationError::OperandNotRegisterError(instr.operand_a.clone()))),
                _ => {}
            }

            // large immediate cannot be out of range due to u16 type limits
            match instr.operand_b {
                Operand::Register(_) | Operand::ShortImmediate(_) => return Err(Box::new(ValidationError::OperandNotLongImmediateError(instr.operand_b.clone()))),
                _ => {}
            }
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::assembler::process_line;
    use super::validate_label;


    #[test]
    fn test_valid_nn_instrs() {
        process_line("  NOP", &HashMap::new(), &mut false);
        process_line("my_label: POPA", &HashMap::new(), &mut false);
        process_line("pusha", &HashMap::new(), &mut false);
        process_line("ret", &HashMap::new(), &mut false);
        process_line("scry", &HashMap::new(), &mut false);
        process_line("CcRy", &HashMap::new(), &mut false);
        process_line("__hello:      Eitr    ", &HashMap::new(), &mut false);
        process_line("Ditr", &HashMap::new(), &mut false);
        process_line("Iret", &HashMap::new(), &mut false);
    }


    #[test]
    fn test_valid_rn_instrs() {
        process_line("ADDC  ax", &HashMap::new(), &mut false);
        process_line("inc bl", &HashMap::new(), &mut false);
        process_line("Subb bh", &HashMap::new(), &mut false);
        process_line("Dec    dx", &HashMap::new(), &mut false);
        process_line("label:  Neg DX", &HashMap::new(), &mut false);
        process_line("_l_a_b_e_l: Push  aH", &HashMap::new(), &mut false);
        process_line("Pop Ah", &HashMap::new(), &mut false);
        process_line("Csign        ah", &HashMap::new(), &mut false);
        process_line("CLEAR rp", &HashMap::new(), &mut false);
   }


    #[test]
    fn test_valid_ri_instrs() {
        process_line("  in rp, 10", &HashMap::new(), &mut false);
        process_line("out ax 10", &HashMap::new(), &mut false);
        process_line("InTr rp, 0", &HashMap::new(), &mut false);
        process_line("lbl: Into, sp,,, 0", &HashMap::new(), &mut false);
    }

    #[test]
    fn test_valid_rl_instrs() {
        process_line("mOvi ax   700", &HashMap::new(), &mut false);
        process_line("mOvi ax   0", &HashMap::new(), &mut false);
    }


    #[test]
    fn test_valid_rr_instrs() {
        process_line("ADD ax bx", &HashMap::new(), &mut false);
        process_line("sub ax bx", &HashMap::new(), &mut false);
        process_line("ADDu ax bx", &HashMap::new(), &mut false);
        process_line("subu ax bx", &HashMap::new(), &mut false);
        process_line("move ah bh", &HashMap::new(), &mut false);
        process_line("And al bl", &HashMap::new(), &mut false);
        process_line("SRa al bl", &HashMap::new(), &mut false);
        process_line("Load ax bx", &HashMap::new(), &mut false);
        process_line("Store ax bx", &HashMap::new(), &mut false);
        process_line("Mul ax bx", &HashMap::new(), &mut false);
        process_line("mulu ax bx", &HashMap::new(), &mut false);
        process_line("div ax, bx", &HashMap::new(), &mut false);
        process_line("divu ax, bx", &HashMap::new(), &mut false);
    }

    #[test]
    #[should_panic]
    fn test_nn_with_reg() {
        process_line("nop ax", &HashMap::new(), &mut false).unwrap();
    }


    #[test]
    #[should_panic]
    fn test_rr_with_one_reg() {
        process_line("add ax", &HashMap::new(), &mut false).unwrap();
    }


    #[test]
    #[should_panic]
    fn test_rr_with_imm() {
        process_line("add ax 10", &HashMap::new(), &mut false).unwrap();
    }


    #[test]
    #[should_panic]
    fn test_rn_with_two_reg() {
        process_line("addc ax sp", &HashMap::new(), &mut false).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_rn_with_imm() {
        process_line("addc 5", &HashMap::new(), &mut false).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_ri_with_no_imm() {
        process_line("out ax", &HashMap::new(), &mut false).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_ri_with_two_reg() {
        process_line("in ax sp", &HashMap::new(), &mut false).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_rl_with_two_reg() {
        process_line("movi ax sp", &HashMap::new(), &mut false).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_rl_with_no_reg() {
        process_line("addc 1000", &HashMap::new(), &mut false).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_mixed_high_low_reg() {
        process_line("add ah, bl", &HashMap::new(), &mut false).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_mixed_size_reg() {
        process_line("add ax, bl", &HashMap::new(), &mut false).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_short_operand_overflow() {
        process_line("in ax 32", &HashMap::new(), &mut false).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_long_operand_overflow() {
        process_line("movi ax 65536", &HashMap::new(), &mut false).unwrap();
    }


    #[test]
    fn test_valid_labels() {
        validate_label("label").unwrap();
        validate_label("__label").unwrap();
        validate_label("__abc__123").unwrap();
        validate_label("_").unwrap();
        validate_label("a").unwrap();
    }

    #[test]
    #[should_panic]
    fn label_starts_with_digit() {
        validate_label("123").unwrap();
    }

    #[test]
    #[should_panic]
    fn label_contains_symbol() {
        validate_label("l@bel").unwrap();
    }

    #[test]
    #[should_panic]
    fn label_contains_space() {
        validate_label("hello world").unwrap();
    }

    #[test]
    #[should_panic]
    fn label_contains_non_ascii() {
        validate_label("aБcd").unwrap();
    }
}
