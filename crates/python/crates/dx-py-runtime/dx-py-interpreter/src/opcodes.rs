//! Opcode definitions for the interpreter

/// Opcode enum matching DPB format
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    // Stack operations (0x00-0x0F)
    Nop = 0x00,
    Pop = 0x01,
    Dup = 0x02,
    DupTwo = 0x03,
    Rot2 = 0x04,
    Rot3 = 0x05,
    Rot4 = 0x06,
    
    // Load operations (0x10-0x1F)
    LoadConst = 0x10,
    LoadName = 0x11,
    LoadFast = 0x12,
    LoadGlobal = 0x13,
    LoadAttr = 0x14,
    LoadMethod = 0x15,
    LoadDeref = 0x16,
    LoadClassDeref = 0x17,
    
    // Store operations (0x20-0x2F)
    StoreName = 0x20,
    StoreFast = 0x21,
    StoreGlobal = 0x22,
    StoreAttr = 0x23,
    StoreDeref = 0x24,
    StoreSubscr = 0x25,
    DeleteName = 0x26,
    DeleteFast = 0x27,
    DeleteGlobal = 0x28,
    DeleteAttr = 0x29,
    DeleteSubscr = 0x2A,
    
    // Binary operations (0x30-0x3F)
    BinaryAdd = 0x30,
    BinarySub = 0x31,
    BinaryMul = 0x32,
    BinaryDiv = 0x33,
    BinaryFloorDiv = 0x34,
    BinaryMod = 0x35,
    BinaryPow = 0x36,
    BinaryMatMul = 0x37,
    BinaryLshift = 0x38,
    BinaryRshift = 0x39,
    BinaryAnd = 0x3A,
    BinaryOr = 0x3B,
    BinaryXor = 0x3C,
    BinarySubscr = 0x3D,
    
    // Unary operations (0x40-0x4F)
    UnaryNot = 0x40,
    UnaryNeg = 0x41,
    UnaryPos = 0x42,
    UnaryInvert = 0x43,
    
    // Comparison operations (0x50-0x5F)
    CompareEq = 0x50,
    CompareNe = 0x51,
    CompareLt = 0x52,
    CompareLe = 0x53,
    CompareGt = 0x54,
    CompareGe = 0x55,
    CompareIs = 0x56,
    CompareIsNot = 0x57,
    CompareIn = 0x58,
    CompareNotIn = 0x59,
}

impl Opcode {
    /// Decode opcode from byte
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Opcode::Nop),
            0x01 => Some(Opcode::Pop),
            0x02 => Some(Opcode::Dup),
            0x03 => Some(Opcode::DupTwo),
            0x04 => Some(Opcode::Rot2),
            0x05 => Some(Opcode::Rot3),
            0x06 => Some(Opcode::Rot4),
            
            0x10 => Some(Opcode::LoadConst),
            0x11 => Some(Opcode::LoadName),
            0x12 => Some(Opcode::LoadFast),
            0x13 => Some(Opcode::LoadGlobal),
            0x14 => Some(Opcode::LoadAttr),
            0x15 => Some(Opcode::LoadMethod),
            0x16 => Some(Opcode::LoadDeref),
            0x17 => Some(Opcode::LoadClassDeref),
            
            0x20 => Some(Opcode::StoreName),
            0x21 => Some(Opcode::StoreFast),
            0x22 => Some(Opcode::StoreGlobal),
            0x23 => Some(Opcode::StoreAttr),
            0x24 => Some(Opcode::StoreDeref),
            0x25 => Some(Opcode::StoreSubscr),
            0x26 => Some(Opcode::DeleteName),
            0x27 => Some(Opcode::DeleteFast),
            0x28 => Some(Opcode::DeleteGlobal),
            0x29 => Some(Opcode::DeleteAttr),
            0x2A => Some(Opcode::DeleteSubscr),
            
            0x30 => Some(Opcode::BinaryAdd),
            0x31 => Some(Opcode::BinarySub),
            0x32 => Some(Opcode::BinaryMul),
            0x33 => Some(Opcode::BinaryDiv),
            0x34 => Some(Opcode::BinaryFloorDiv),
            0x35 => Some(Opcode::BinaryMod),
            0x36 => Some(Opcode::BinaryPow),
            0x37 => Some(Opcode::BinaryMatMul),
            0x38 => Some(Opcode::BinaryLshift),
            0x39 => Some(Opcode::BinaryRshift),
            0x3A => Some(Opcode::BinaryAnd),
            0x3B => Some(Opcode::BinaryOr),
            0x3C => Some(Opcode::BinaryXor),
            0x3D => Some(Opcode::BinarySubscr),
            
            0x40 => Some(Opcode::UnaryNot),
            0x41 => Some(Opcode::UnaryNeg),
            0x42 => Some(Opcode::UnaryPos),
            0x43 => Some(Opcode::UnaryInvert),
            
            0x50 => Some(Opcode::CompareEq),
            0x51 => Some(Opcode::CompareNe),
            0x52 => Some(Opcode::CompareLt),
            0x53 => Some(Opcode::CompareLe),
            0x54 => Some(Opcode::CompareGt),
            0x55 => Some(Opcode::CompareGe),
            0x56 => Some(Opcode::CompareIs),
            0x57 => Some(Opcode::CompareIsNot),
            0x58 => Some(Opcode::CompareIn),
            0x59 => Some(Opcode::CompareNotIn),
            
            _ => None,
        }
    }
    
    /// Check if opcode has an argument
    pub fn has_arg(&self) -> bool {
        matches!(
            self,
            Opcode::LoadConst
                | Opcode::LoadName
                | Opcode::LoadFast
                | Opcode::LoadGlobal
                | Opcode::LoadAttr
                | Opcode::LoadMethod
                | Opcode::LoadDeref
                | Opcode::LoadClassDeref
                | Opcode::StoreName
                | Opcode::StoreFast
                | Opcode::StoreGlobal
                | Opcode::StoreAttr
                | Opcode::StoreDeref
                | Opcode::DeleteName
                | Opcode::DeleteFast
                | Opcode::DeleteGlobal
                | Opcode::DeleteAttr
        )
    }
}
