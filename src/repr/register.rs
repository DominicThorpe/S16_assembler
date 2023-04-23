#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Register {
    None, // no register
    Ax, Al, Ah, // primary accumulator
    Bx, Bl, Bh, // secondary accumulator
    Cx, Cl, Ch, // counter
    Dx, Dl, Dh, // auxilliary
    Rp, // return pointer
    Fp, // frame pointer
    Bp, // base pointer
    Sp, // stack pointer
    St, // status flags
    Pc  // program counter
}

impl Into<u16> for Register {
    fn into(self) -> u16 {
        match self {
            Register::None => 0,
            Register::Ax | Register::Al | Register::Ah => 0,
            Register::Bx | Register::Bl | Register::Bh => 1,
            Register::Cx | Register::Cl | Register::Ch => 2,
            Register::Dx | Register::Dl | Register::Dh => 3,
            Register::Rp => 4,
            Register::Fp => 5,
            Register::Bp => 6,
            Register::Sp => 7,
            _ => panic!("Cannot convert register to u8")
        }
    }
}

impl Into<String> for Register {
    fn into(self) -> String {
        let reg_str = match self {
            Register::None => "none",
            Register::Ax => "ax",
            Register::Al => "al",
            Register::Ah => "ah",
            Register::Bx => "bx",
            Register::Bl => "bl",
            Register::Bh => "bh",
            Register::Cx => "cx",
            Register::Cl => "cl",
            Register::Ch => "ch",
            Register::Dx => "dx",
            Register::Dl => "dl",
            Register::Dh => "dh",
            Register::Rp => "rp",
            Register::Fp => "fp",
            Register::Bp => "bp",
            Register::Sp => "sp",
            Register::St => "st",
            Register::Pc => "pc"
        };

        String::from(reg_str)
    }
}

impl From<&String> for Register {
    fn from(reg:&String) -> Register {
        match reg.to_lowercase().as_str() {
            "none" => Register::None,
            "ax" => Register::Ax,
            "ah" => Register::Ah,
            "al" => Register::Al,
            "bx" => Register::Bx,
            "bh" => Register::Bh,
            "bl" => Register::Bl,
            "cx" => Register::Cx,
            "ch" => Register::Ch,
            "cl" => Register::Cl,
            "dx" => Register::Dx,
            "dh" => Register::Dh,
            "dl" => Register::Dl,
            "rp" => Register::Rp,
            "fp" => Register::Fp,
            "bp" => Register::Bp,
            "sp" => Register::Sp,
            "pc" => Register::Pc,
            _ => panic!("Invalid register {} found", reg)
        }
    }
}

impl Register {
    /**
     * Returns true if the register requires the high bit of the instruction to be set.
     */
    pub fn is_high_reg(&self) -> bool {
        match self {
            Register::Ax | Register::Ah | Register::Bx | Register::Bh | Register::Cx | Register::Ch
             | Register::Dx | Register::Dh | Register::Bp | Register::Fp | Register::Rp 
             | Register::Sp => true,
            _ => false
        }
    }


    /**
     * Returns true if the register requires the low bit of the instruction to be set.
     */
    pub fn is_low_reg(&self) -> bool {
        match self {
            Register::Ax | Register::Al | Register::Bx | Register::Bl | Register::Cx | Register::Cl
             | Register::Dx | Register::Dl | Register::Bp | Register::Fp | Register::Rp 
             | Register::Sp => true,
            _ => false
        }
    }
}



#[cfg(test)]
mod tests {
    use super::Register;


    #[test]
    fn test_into_int() {
        let index:u16 = Register::Ax.into();
        assert_eq!(index, 0);

        let index:u16 = Register::Bx.into();
        assert_eq!(index, 1);

        let index:u16 = Register::Fp.into();
        assert_eq!(index, 5);
    }


    #[test]
    #[should_panic]
    fn test_invalid_into_int() {
        let _:u16 = Register::Pc.into();
    }
}
