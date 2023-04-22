/**
 * Represents the full range of opcodes available to the Sim6 processor
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    Nop, // Do nothing
    Add, // Rd = Rd + Rt (signed)
    Addu, // Rd = Rd + Rt (unsigned)
    Addc, // Rd = Rd + Flags[Carry]
    Inc, // RD = Rd + 1
    Sub, // Rd = Rd - Rt (signed)
    Subu, // Rd = Rd - Rt (unsigned)
    Subb, // Rd = Rd - Flags[Carry]
    Dec, // Rd = Rd - 1
    Cmp, // Set flags for result of Rd - Rt
    Neg, // Rd = -Rd (2s complement)
    Move, // Rd = Rt
    Push, // Push Rd to stack
    Pop, // Pop stack to Rd
    PushA, // Push all registers to stack
    PopA, // Pop all registers from stack
    PushF, // Push flags to stack
    PopF, // Pop flags from stack
    Swap, // Rd = Rt; Rt = Rd
    In, // Push Rd to port[imm]
    Out, // Move val in port[imm]
    Lda, // Load address of label
    MovI, // Push word to register
    Mul, // Rd = Rth * Rtl (signed)
    Mulu, // Rd = Rth * Rtl (unsigned)
    Div, // Rd = Rth / Rtl (signed)
    Divu, // Rd = Rth / Rtl (unsigned)
    Csign, // Sign extend Rdl into Rdh
    Not, // Rd = ~Rd
    And, // Rd = Rd & Rt
    Or, // Rd = Rd | Rt
    Xor, // Rd = Rd ^ Rt
    Sra, // Rd = Rd >> Rt
    Srl, // Rd = Rd >>> Rt
    Sll, // Rd = Rd >> Rt
    Clear, // Rd = 0
    Call, // function call (addr in Rd)
    Ret, // return from func call
    Jump, // jump to Rd
    Jeq, // jump to Rd if flags[zero]
    Jne, // jump to Rd if not flags[zero]
    Jgt, // jump to Rd if not flags[sign] 
    Jle, // jump to Rd if flags[sign]
    Jgte, // jump to Rd if not flags[sign] or flags[zero]
    Jlte, // jump to Rd if flags[sign] or flags[zero]
    Jzro, // jump to Rd if flags[zero]
    Jnzro, // jump to Rd if not flags[zero]
    Jovf, // jump to Rd if flags[overflow]
    Jcry, // jump to Rd if flags[carry]
    Scry, // flags[carry] = 1
    Ccry, // flags[carry] = 0
    Eitr, // enable interrupts
    Ditr, // disable interrupts
    Intr, // Call interrupt code imm
    Into, // Interrupt code imm if flags[overflow]
    Iret, // return from interrupt
    Load, // load value at address in Rt into Rd
    Store // store value in Rd into address in Rt
}

impl Into<u16> for Opcode {
    /**
     * Converts an opcode to its 6-bit integer representation
     */
    fn into(self) -> u16 {
        match self {
            Opcode::Nop    => 0,
            Opcode::Add    => 1,
            Opcode::Addu   => 2,
            Opcode::Addc   => 3,
            Opcode::Inc    => 4,
            Opcode::Sub    => 5,
            Opcode::Subu   => 6,
            Opcode::Subb   => 7,
            Opcode::Dec    => 8,
            Opcode::Cmp    => 9,
            Opcode::Neg    => 10,
            Opcode::Move   => 11,
            Opcode::Push   => 12,
            Opcode::Pop    => 13,
            Opcode::PushA  => 14,
            Opcode::PopA   => 15,
            Opcode::PushF  => 16,
            Opcode::PopF   => 17,
            Opcode::Swap   => 18,
            Opcode::In     => 19,
            Opcode::Out    => 20,
            Opcode::Lda    => 21,
            Opcode::MovI   => 22,
            Opcode::Mul    => 23,
            Opcode::Mulu   => 24,
            Opcode::Div    => 25,
            Opcode::Divu   => 26,
            Opcode::Csign  => 27,
            Opcode::Not    => 28,
            Opcode::And    => 29,
            Opcode::Or     => 30,
            Opcode::Xor    => 31,
            Opcode::Sra    => 32,
            Opcode::Srl    => 33,
            Opcode::Sll    => 34,
            Opcode::Clear  => 35,
            Opcode::Call   => 36,
            Opcode::Ret    => 37,
            Opcode::Jump   => 38,
            Opcode::Jeq    => 39,
            Opcode::Jne    => 40,
            Opcode::Jgt    => 41,
            Opcode::Jle    => 42,
            Opcode::Jgte   => 43,
            Opcode::Jlte   => 44,
            Opcode::Jzro   => 45,
            Opcode::Jnzro  => 46,
            Opcode::Jovf   => 47,
            Opcode::Jcry   => 48,
            Opcode::Scry   => 49,
            Opcode::Ccry   => 50,
            Opcode::Eitr   => 51,
            Opcode::Ditr   => 52,
            Opcode::Intr   => 53,
            Opcode::Into   => 54,
            Opcode::Iret   => 55,
            Opcode::Load   => 56,
            Opcode::Store  => 57,
        } 
    }
}

impl From<&String> for Opcode {
    /**
     * Translates a string to the opcode it represents, is case-insensitive, panics if
     * it finds an invalid opcode.
     */
    fn from(code:&String) -> Opcode {
        match code.to_lowercase().as_str() {
            "nop"   => Opcode::Nop,
            "add"   => Opcode::Add,
            "addu"  => Opcode::Addu,
            "addc"  => Opcode::Addc,
            "inc"   => Opcode::Inc,
            "sub"   => Opcode::Sub,
            "subu"  => Opcode::Subu,
            "subb"  => Opcode::Subb,
            "dec"   => Opcode::Dec,
            "cmp"   => Opcode::Cmp,
            "neg"   => Opcode::Neg,
            "move"  => Opcode::Move,
            "push"  => Opcode::Push,
            "pop"   => Opcode::Pop,
            "pusha" => Opcode::PushA,
            "popa"  => Opcode::PopA,
            "pushf" => Opcode::PushF,
            "popf"  => Opcode::PopF,
            "swap"  => Opcode::Swap,
            "in"    => Opcode::In,
            "out"   => Opcode::Out,
            "lda"   => Opcode::Lda,
            "movi"  => Opcode::MovI,
            "mul"   => Opcode::Mul,
            "mulu"  => Opcode::Mulu,
            "div"   => Opcode::Div,
            "divu"  => Opcode::Divu,
            "csign" => Opcode::Csign,
            "not"   => Opcode::Not,
            "and"   => Opcode::And,
            "or"    => Opcode::Or,
            "xor"   => Opcode::Xor,
            "sra"   => Opcode::Sra,
            "srl"   => Opcode::Srl,
            "sll"   => Opcode::Sll,
            "clear" => Opcode::Clear,
            "call"  => Opcode::Call,
            "ret"   => Opcode::Ret,
            "jump"  => Opcode::Jump,
            "jeq"   => Opcode::Jeq,
            "jne"   => Opcode::Jne,
            "jgt"   => Opcode::Jgt,
            "jle"   => Opcode::Jle,
            "jgte"  => Opcode::Jgte,
            "jlte"  => Opcode::Jlte,
            "jzro"  => Opcode::Jzro,
            "jnzro" => Opcode::Jnzro,
            "jovf"  => Opcode::Jovf,
            "jcry"  => Opcode::Jcry,
            "scry"  => Opcode::Scry,
            "ccry"  => Opcode::Ccry,
            "eitr"  => Opcode::Eitr,
            "ditr"  => Opcode::Ditr,
            "intr"  => Opcode::Intr,
            "into"  => Opcode::Into,
            "iret"  => Opcode::Iret,
            "load"  => Opcode::Load,
            "store" => Opcode::Store,  
            _ => panic!("Invalid opcode found")
        }
    }
}
