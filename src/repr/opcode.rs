/**
 * Represents the full range of opcodes available to the Sim6 processor
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    Nop, // Do nothing
    Add, // Rd = Rd + Rt
    Addc, // Rd = Rd + Flags[Carry]
    Inc, // RD = Rd + 1
    Sub, // Rd = Rd - Rt
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
    Mul, // Rd = Rth * Rtl (unsigned)
    Imul, // Rd = Rth * Rtl (signed)
    Div, // Rd = Rth / Rtl (unsigned)
    Idiv, // Rd = Rth / Rtl (signed)
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
            Opcode::Addc   => 2,
            Opcode::Inc    => 3,
            Opcode::Sub    => 4,
            Opcode::Subb   => 5,
            Opcode::Dec    => 6,
            Opcode::Cmp    => 7,
            Opcode::Neg    => 8,
            Opcode::Move   => 9,
            Opcode::Push   => 10,
            Opcode::Pop    => 11,
            Opcode::PushA  => 12,
            Opcode::PopA   => 13,
            Opcode::PushF  => 14,
            Opcode::PopF   => 15,
            Opcode::Swap   => 16,
            Opcode::In     => 17,
            Opcode::Out    => 18,
            Opcode::Lda    => 19,
            Opcode::MovI   => 20,
            Opcode::Mul    => 21,
            Opcode::Imul   => 22,
            Opcode::Div    => 23,
            Opcode::Idiv   => 24,
            Opcode::Csign  => 25,
            Opcode::Not    => 26,
            Opcode::And    => 27,
            Opcode::Or     => 28,
            Opcode::Xor    => 29,
            Opcode::Sra    => 30,
            Opcode::Srl    => 31,
            Opcode::Sll    => 32,
            Opcode::Clear  => 33,
            Opcode::Call   => 34,
            Opcode::Ret    => 35,
            Opcode::Jump   => 36,
            Opcode::Jeq    => 37,
            Opcode::Jne    => 38,
            Opcode::Jgt    => 39,
            Opcode::Jle    => 40,
            Opcode::Jgte   => 41,
            Opcode::Jlte   => 42,
            Opcode::Jzro   => 43,
            Opcode::Jnzro  => 44,
            Opcode::Jovf   => 45,
            Opcode::Jcry   => 46,
            Opcode::Scry   => 47,
            Opcode::Ccry   => 48,
            Opcode::Eitr   => 49,
            Opcode::Ditr   => 50,
            Opcode::Intr   => 51,
            Opcode::Into   => 52,
            Opcode::Iret   => 53,
            Opcode::Load   => 52,
            Opcode::Store  => 53,
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
            "addc"  => Opcode::Addc,
            "inc"   => Opcode::Inc,
            "sub"   => Opcode::Sub,
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
            "imul"  => Opcode::Imul,
            "div"   => Opcode::Div,
            "idiv"  => Opcode::Idiv,
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
