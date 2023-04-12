#[allow(dead_code)]
#[derive(Debug)]
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
            _ => panic!("Invalid register found")
        }
    }
}

impl Register {
    pub fn get_reg_code(reg_a:&Register, reg_b:&Register) -> u16 {
        let reg_a_code = match reg_a {
            Register::None => 0b0000,
            Register::Al | Register::Bl | Register::Cl | Register::Dl => 0b1000,
            Register::Ah | Register::Bh | Register::Ch | Register::Dh => 0b0100,
            Register::Ax | Register::Bx | Register::Cx | Register::Dx | Register::Rp | Register::Fp | Register::Bp | Register::Sp => 0b1100,
            _ => panic!("Invalid register upper found")
        };

        let reg_b_code = match reg_b {
            Register::None => 0b0000,
            Register::Al | Register::Bl | Register::Cl | Register::Dl => 0b0010,
            Register::Ah | Register::Bh | Register::Ch | Register::Dh => 0b0001,
            Register::Ax | Register::Bx | Register::Cx | Register::Dx | Register::Rp | Register::Fp | Register::Bp | Register::Sp => 0b0011,
            _ => panic!("Invalid lower register found")
        };

        let result = reg_a_code | reg_b_code;
        result
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


    #[test]
    fn get_reg_code() {
        assert_eq!(Register::get_reg_code(&Register::Ax, &Register::Bx), 0b1111);
        assert_eq!(Register::get_reg_code(&Register::Rp, &Register::Fp), 0b1111);

        assert_eq!(Register::get_reg_code(&Register::Ax, &Register::None), 0b1100);
        assert_eq!(Register::get_reg_code(&Register::None, &Register::Dx), 0b0011);

        assert_eq!(Register::get_reg_code(&Register::Al, &Register::Bx), 0b1011);
        assert_eq!(Register::get_reg_code(&Register::Ah, &Register::Bx), 0b0111);
        assert_eq!(Register::get_reg_code(&Register::Ax, &Register::Bl), 0b1110);
        assert_eq!(Register::get_reg_code(&Register::Ax, &Register::Bh), 0b1101);

        assert_eq!(Register::get_reg_code(&Register::Al, &Register::Bl), 0b1010);
        assert_eq!(Register::get_reg_code(&Register::Ah, &Register::Bh), 0b0101);
        assert_eq!(Register::get_reg_code(&Register::Ah, &Register::Bl), 0b0110);
        assert_eq!(Register::get_reg_code(&Register::Al, &Register::Bh), 0b1001);
    }
}
