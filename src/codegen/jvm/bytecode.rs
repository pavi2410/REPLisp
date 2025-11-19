/// JVM bytecode instructions for Java 8
/// Based on the Java Virtual Machine Specification (Java SE 8 Edition)

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    // Constants
    Aconst_null,
    Iconst_m1,
    Iconst_0,
    Iconst_1,
    Iconst_2,
    Iconst_3,
    Iconst_4,
    Iconst_5,
    Lconst_0,
    Lconst_1,
    Fconst_0,
    Fconst_1,
    Fconst_2,
    Dconst_0,
    Dconst_1,
    Bipush(i8),
    Sipush(i16),
    Ldc(u8),
    Ldc_w(u16),
    Ldc2_w(u16),

    // Loads
    Iload(u8),
    Lload(u8),
    Fload(u8),
    Dload(u8),
    Aload(u8),
    Iload_0,
    Iload_1,
    Iload_2,
    Iload_3,
    Lload_0,
    Lload_1,
    Lload_2,
    Lload_3,
    Fload_0,
    Fload_1,
    Fload_2,
    Fload_3,
    Dload_0,
    Dload_1,
    Dload_2,
    Dload_3,
    Aload_0,
    Aload_1,
    Aload_2,
    Aload_3,

    // Stores
    Istore(u8),
    Lstore(u8),
    Fstore(u8),
    Dstore(u8),
    Astore(u8),
    Istore_0,
    Istore_1,
    Istore_2,
    Istore_3,
    Astore_0,
    Astore_1,
    Astore_2,
    Astore_3,

    // Stack
    Pop,
    Pop2,
    Dup,
    Dup_x1,
    Dup_x2,
    Dup2,
    Dup2_x1,
    Dup2_x2,
    Swap,

    // Math
    Iadd,
    Ladd,
    Fadd,
    Dadd,
    Isub,
    Lsub,
    Fsub,
    Dsub,
    Imul,
    Lmul,
    Fmul,
    Dmul,
    Idiv,
    Ldiv,
    Fdiv,
    Ddiv,
    Irem,
    Lrem,
    Frem,
    Drem,
    Ineg,
    Lneg,
    Fneg,
    Dneg,

    // Conversions
    I2l,
    I2f,
    I2d,
    L2i,
    L2f,
    L2d,
    F2i,
    F2l,
    F2d,
    D2i,
    D2l,
    D2f,
    I2b,
    I2c,
    I2s,

    // Comparisons
    Lcmp,
    Fcmpl,
    Fcmpg,
    Dcmpl,
    Dcmpg,
    Ifeq(i16),
    Ifne(i16),
    Iflt(i16),
    Ifge(i16),
    Ifgt(i16),
    Ifle(i16),
    If_icmpeq(i16),
    If_icmpne(i16),
    If_icmplt(i16),
    If_icmpge(i16),
    If_icmpgt(i16),
    If_icmple(i16),
    If_acmpeq(i16),
    If_acmpne(i16),

    // Control
    Goto(i16),
    Jsr(i16),
    Ret(u8),
    Tableswitch,
    Lookupswitch,
    Ireturn,
    Lreturn,
    Freturn,
    Dreturn,
    Areturn,
    Return,

    // References
    Getstatic(u16),
    Putstatic(u16),
    Getfield(u16),
    Putfield(u16),
    Invokevirtual(u16),
    Invokespecial(u16),
    Invokestatic(u16),
    Invokeinterface(u16, u8),
    Invokedynamic(u16),
    New(u16),
    Newarray(u8),
    Anewarray(u16),
    Arraylength,
    Athrow,
    Checkcast(u16),
    Instanceof(u16),

    // Extended
    Wide,
    Multianewarray(u16, u8),
    Ifnull(i16),
    Ifnonnull(i16),
    Goto_w(i32),
    Jsr_w(i32),
}

impl Opcode {
    /// Get the bytecode value for this opcode
    pub fn to_byte(&self) -> u8 {
        match self {
            Opcode::Aconst_null => 0x01,
            Opcode::Iconst_m1 => 0x02,
            Opcode::Iconst_0 => 0x03,
            Opcode::Iconst_1 => 0x04,
            Opcode::Iconst_2 => 0x05,
            Opcode::Iconst_3 => 0x06,
            Opcode::Iconst_4 => 0x07,
            Opcode::Iconst_5 => 0x08,
            Opcode::Lconst_0 => 0x09,
            Opcode::Lconst_1 => 0x0a,
            Opcode::Fconst_0 => 0x0b,
            Opcode::Fconst_1 => 0x0c,
            Opcode::Fconst_2 => 0x0d,
            Opcode::Dconst_0 => 0x0e,
            Opcode::Dconst_1 => 0x0f,
            Opcode::Bipush(_) => 0x10,
            Opcode::Sipush(_) => 0x11,
            Opcode::Ldc(_) => 0x12,
            Opcode::Ldc_w(_) => 0x13,
            Opcode::Ldc2_w(_) => 0x14,
            Opcode::Iload(_) => 0x15,
            Opcode::Lload(_) => 0x16,
            Opcode::Fload(_) => 0x17,
            Opcode::Dload(_) => 0x18,
            Opcode::Aload(_) => 0x19,
            Opcode::Iload_0 => 0x1a,
            Opcode::Iload_1 => 0x1b,
            Opcode::Iload_2 => 0x1c,
            Opcode::Iload_3 => 0x1d,
            Opcode::Lload_0 => 0x1e,
            Opcode::Lload_1 => 0x1f,
            Opcode::Lload_2 => 0x20,
            Opcode::Lload_3 => 0x21,
            Opcode::Fload_0 => 0x22,
            Opcode::Fload_1 => 0x23,
            Opcode::Fload_2 => 0x24,
            Opcode::Fload_3 => 0x25,
            Opcode::Dload_0 => 0x26,
            Opcode::Dload_1 => 0x27,
            Opcode::Dload_2 => 0x28,
            Opcode::Dload_3 => 0x29,
            Opcode::Aload_0 => 0x2a,
            Opcode::Aload_1 => 0x2b,
            Opcode::Aload_2 => 0x2c,
            Opcode::Aload_3 => 0x2d,
            Opcode::Istore(_) => 0x36,
            Opcode::Lstore(_) => 0x37,
            Opcode::Fstore(_) => 0x38,
            Opcode::Dstore(_) => 0x39,
            Opcode::Astore(_) => 0x3a,
            Opcode::Istore_0 => 0x3b,
            Opcode::Istore_1 => 0x3c,
            Opcode::Istore_2 => 0x3d,
            Opcode::Istore_3 => 0x3e,
            Opcode::Astore_0 => 0x4b,
            Opcode::Astore_1 => 0x4c,
            Opcode::Astore_2 => 0x4d,
            Opcode::Astore_3 => 0x4e,
            Opcode::Pop => 0x57,
            Opcode::Pop2 => 0x58,
            Opcode::Dup => 0x59,
            Opcode::Dup_x1 => 0x5a,
            Opcode::Dup_x2 => 0x5b,
            Opcode::Dup2 => 0x5c,
            Opcode::Dup2_x1 => 0x5d,
            Opcode::Dup2_x2 => 0x5e,
            Opcode::Swap => 0x5f,
            Opcode::Iadd => 0x60,
            Opcode::Ladd => 0x61,
            Opcode::Fadd => 0x62,
            Opcode::Dadd => 0x63,
            Opcode::Isub => 0x64,
            Opcode::Lsub => 0x65,
            Opcode::Fsub => 0x66,
            Opcode::Dsub => 0x67,
            Opcode::Imul => 0x68,
            Opcode::Lmul => 0x69,
            Opcode::Fmul => 0x6a,
            Opcode::Dmul => 0x6b,
            Opcode::Idiv => 0x6c,
            Opcode::Ldiv => 0x6d,
            Opcode::Fdiv => 0x6e,
            Opcode::Ddiv => 0x6f,
            Opcode::Irem => 0x70,
            Opcode::Lrem => 0x71,
            Opcode::Frem => 0x72,
            Opcode::Drem => 0x73,
            Opcode::Ineg => 0x74,
            Opcode::Lneg => 0x75,
            Opcode::Fneg => 0x76,
            Opcode::Dneg => 0x77,
            Opcode::I2l => 0x85,
            Opcode::I2f => 0x86,
            Opcode::I2d => 0x87,
            Opcode::L2i => 0x88,
            Opcode::L2f => 0x89,
            Opcode::L2d => 0x8a,
            Opcode::F2i => 0x8b,
            Opcode::F2l => 0x8c,
            Opcode::F2d => 0x8d,
            Opcode::D2i => 0x8e,
            Opcode::D2l => 0x8f,
            Opcode::D2f => 0x90,
            Opcode::I2b => 0x91,
            Opcode::I2c => 0x92,
            Opcode::I2s => 0x93,
            Opcode::Lcmp => 0x94,
            Opcode::Fcmpl => 0x95,
            Opcode::Fcmpg => 0x96,
            Opcode::Dcmpl => 0x97,
            Opcode::Dcmpg => 0x98,
            Opcode::Ifeq(_) => 0x99,
            Opcode::Ifne(_) => 0x9a,
            Opcode::Iflt(_) => 0x9b,
            Opcode::Ifge(_) => 0x9c,
            Opcode::Ifgt(_) => 0x9d,
            Opcode::Ifle(_) => 0x9e,
            Opcode::If_icmpeq(_) => 0x9f,
            Opcode::If_icmpne(_) => 0xa0,
            Opcode::If_icmplt(_) => 0xa1,
            Opcode::If_icmpge(_) => 0xa2,
            Opcode::If_icmpgt(_) => 0xa3,
            Opcode::If_icmple(_) => 0xa4,
            Opcode::If_acmpeq(_) => 0xa5,
            Opcode::If_acmpne(_) => 0xa6,
            Opcode::Goto(_) => 0xa7,
            Opcode::Jsr(_) => 0xa8,
            Opcode::Ret(_) => 0xa9,
            Opcode::Tableswitch => 0xaa,
            Opcode::Lookupswitch => 0xab,
            Opcode::Ireturn => 0xac,
            Opcode::Lreturn => 0xad,
            Opcode::Freturn => 0xae,
            Opcode::Dreturn => 0xaf,
            Opcode::Areturn => 0xb0,
            Opcode::Return => 0xb1,
            Opcode::Getstatic(_) => 0xb2,
            Opcode::Putstatic(_) => 0xb3,
            Opcode::Getfield(_) => 0xb4,
            Opcode::Putfield(_) => 0xb5,
            Opcode::Invokevirtual(_) => 0xb6,
            Opcode::Invokespecial(_) => 0xb7,
            Opcode::Invokestatic(_) => 0xb8,
            Opcode::Invokeinterface(_, _) => 0xb9,
            Opcode::Invokedynamic(_) => 0xba,
            Opcode::New(_) => 0xbb,
            Opcode::Newarray(_) => 0xbc,
            Opcode::Anewarray(_) => 0xbd,
            Opcode::Arraylength => 0xbe,
            Opcode::Athrow => 0xbf,
            Opcode::Checkcast(_) => 0xc0,
            Opcode::Instanceof(_) => 0xc1,
            Opcode::Wide => 0xc4,
            Opcode::Multianewarray(_, _) => 0xc5,
            Opcode::Ifnull(_) => 0xc6,
            Opcode::Ifnonnull(_) => 0xc7,
            Opcode::Goto_w(_) => 0xc8,
            Opcode::Jsr_w(_) => 0xc9,
        }
    }

    /// Encode the full instruction (opcode + operands) to bytes
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![self.to_byte()];

        match self {
            Opcode::Bipush(val) => bytes.push(*val as u8),
            Opcode::Sipush(val) => {
                bytes.push((val >> 8) as u8);
                bytes.push(*val as u8);
            }
            Opcode::Ldc(idx) => bytes.push(*idx),
            Opcode::Ldc_w(idx) | Opcode::Ldc2_w(idx) => {
                bytes.push((idx >> 8) as u8);
                bytes.push(*idx as u8);
            }
            Opcode::Iload(idx) | Opcode::Lload(idx) | Opcode::Fload(idx) |
            Opcode::Dload(idx) | Opcode::Aload(idx) => bytes.push(*idx),
            Opcode::Istore(idx) | Opcode::Lstore(idx) | Opcode::Fstore(idx) |
            Opcode::Dstore(idx) | Opcode::Astore(idx) => bytes.push(*idx),
            Opcode::Ifeq(offset) | Opcode::Ifne(offset) | Opcode::Iflt(offset) |
            Opcode::Ifge(offset) | Opcode::Ifgt(offset) | Opcode::Ifle(offset) |
            Opcode::If_icmpeq(offset) | Opcode::If_icmpne(offset) | Opcode::If_icmplt(offset) |
            Opcode::If_icmpge(offset) | Opcode::If_icmpgt(offset) | Opcode::If_icmple(offset) |
            Opcode::If_acmpeq(offset) | Opcode::If_acmpne(offset) |
            Opcode::Goto(offset) | Opcode::Jsr(offset) |
            Opcode::Ifnull(offset) | Opcode::Ifnonnull(offset) => {
                bytes.push((offset >> 8) as u8);
                bytes.push(*offset as u8);
            }
            Opcode::Ret(idx) => bytes.push(*idx),
            Opcode::Getstatic(idx) | Opcode::Putstatic(idx) | Opcode::Getfield(idx) |
            Opcode::Putfield(idx) | Opcode::Invokevirtual(idx) | Opcode::Invokespecial(idx) |
            Opcode::Invokestatic(idx) | Opcode::Invokedynamic(idx) | Opcode::New(idx) |
            Opcode::Anewarray(idx) | Opcode::Checkcast(idx) | Opcode::Instanceof(idx) => {
                bytes.push((idx >> 8) as u8);
                bytes.push(*idx as u8);
            }
            Opcode::Invokeinterface(idx, count) => {
                bytes.push((idx >> 8) as u8);
                bytes.push(*idx as u8);
                bytes.push(*count);
                bytes.push(0); // reserved
            }
            Opcode::Newarray(atype) => bytes.push(*atype),
            Opcode::Multianewarray(idx, dims) => {
                bytes.push((idx >> 8) as u8);
                bytes.push(*idx as u8);
                bytes.push(*dims);
            }
            Opcode::Goto_w(offset) | Opcode::Jsr_w(offset) => {
                bytes.push((offset >> 24) as u8);
                bytes.push((offset >> 16) as u8);
                bytes.push((offset >> 8) as u8);
                bytes.push(*offset as u8);
            }
            _ => {} // No operands
        }

        bytes
    }
}
