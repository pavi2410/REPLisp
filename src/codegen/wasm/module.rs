/// WebAssembly module structure
///
/// This module defines the structure of a WebAssembly module according to the
/// WebAssembly binary format specification.

use super::encoder::WasmEncoder;

/// WebAssembly value types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
}

impl ValType {
    pub fn to_byte(&self) -> u8 {
        match self {
            ValType::I32 => 0x7F,
            ValType::I64 => 0x7E,
            ValType::F32 => 0x7D,
            ValType::F64 => 0x7C,
        }
    }
}

/// Function type (signature)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncType {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
}

impl FuncType {
    pub fn new(params: Vec<ValType>, results: Vec<ValType>) -> Self {
        Self { params, results }
    }
}

/// Export kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportKind {
    Func,
    Table,
    Mem,
    Global,
}

impl ExportKind {
    pub fn to_byte(&self) -> u8 {
        match self {
            ExportKind::Func => 0x00,
            ExportKind::Table => 0x01,
            ExportKind::Mem => 0x02,
            ExportKind::Global => 0x03,
        }
    }
}

/// Export descriptor
#[derive(Debug, Clone)]
pub struct Export {
    pub name: String,
    pub kind: ExportKind,
    pub index: u32,
}

impl Export {
    pub fn func(name: String, index: u32) -> Self {
        Self {
            name,
            kind: ExportKind::Func,
            index,
        }
    }

    pub fn memory(name: String, index: u32) -> Self {
        Self {
            name,
            kind: ExportKind::Mem,
            index,
        }
    }
}

/// Memory limits
#[derive(Debug, Clone)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

impl Limits {
    pub fn min_only(min: u32) -> Self {
        Self { min, max: None }
    }

    pub fn with_max(min: u32, max: u32) -> Self {
        Self {
            min,
            max: Some(max),
        }
    }
}

/// Memory descriptor
#[derive(Debug, Clone)]
pub struct Memory {
    pub limits: Limits,
}

impl Memory {
    pub fn new(limits: Limits) -> Self {
        Self { limits }
    }
}

/// Global mutability
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mutability {
    Const,
    Var,
}

impl Mutability {
    pub fn to_byte(&self) -> u8 {
        match self {
            Mutability::Const => 0x00,
            Mutability::Var => 0x01,
        }
    }
}

/// Global type
#[derive(Debug, Clone)]
pub struct GlobalType {
    pub val_type: ValType,
    pub mutability: Mutability,
}

impl GlobalType {
    pub fn new(val_type: ValType, mutability: Mutability) -> Self {
        Self {
            val_type,
            mutability,
        }
    }
}

/// Global variable
#[derive(Debug, Clone)]
pub struct Global {
    pub global_type: GlobalType,
    pub init_expr: Vec<u8>,
}

impl Global {
    pub fn new(global_type: GlobalType, init_expr: Vec<u8>) -> Self {
        Self {
            global_type,
            init_expr,
        }
    }
}

/// Function code (locals + body)
#[derive(Debug, Clone)]
pub struct FunctionCode {
    pub locals: Vec<(u32, ValType)>, // (count, type) pairs
    pub body: Vec<u8>,
}

impl FunctionCode {
    pub fn new(locals: Vec<(u32, ValType)>, body: Vec<u8>) -> Self {
        Self { locals, body }
    }
}

/// Import descriptor kind
#[derive(Debug, Clone)]
pub enum ImportKind {
    Func(u32),       // type index
    Memory(Limits),
}

/// Import descriptor
#[derive(Debug, Clone)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub kind: ImportKind,
}

impl Import {
    pub fn func(module: String, name: String, type_idx: u32) -> Self {
        Self {
            module,
            name,
            kind: ImportKind::Func(type_idx),
        }
    }

    pub fn memory(module: String, name: String, limits: Limits) -> Self {
        Self {
            module,
            name,
            kind: ImportKind::Memory(limits),
        }
    }
}

/// WebAssembly module
#[derive(Debug, Clone)]
pub struct WasmModule {
    pub types: Vec<FuncType>,
    pub imports: Vec<Import>,
    pub functions: Vec<u32>,  // type indices
    pub memories: Vec<Memory>,
    pub globals: Vec<Global>,
    pub exports: Vec<Export>,
    pub code: Vec<FunctionCode>,
}

impl WasmModule {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            imports: Vec::new(),
            functions: Vec::new(),
            memories: Vec::new(),
            globals: Vec::new(),
            exports: Vec::new(),
            code: Vec::new(),
        }
    }

    /// Add a function type and return its index
    pub fn add_type(&mut self, func_type: FuncType) -> u32 {
        // Check if this type already exists
        for (i, existing) in self.types.iter().enumerate() {
            if existing == &func_type {
                return i as u32;
            }
        }
        // Add new type
        let idx = self.types.len() as u32;
        self.types.push(func_type);
        idx
    }

    /// Add an import
    pub fn add_import(&mut self, import: Import) {
        self.imports.push(import);
    }

    /// Add a function with the given type index
    pub fn add_function(&mut self, type_idx: u32, code: FunctionCode) -> u32 {
        let func_idx = (self.imports.iter().filter(|imp| matches!(imp.kind, ImportKind::Func(_))).count()
                       + self.functions.len()) as u32;
        self.functions.push(type_idx);
        self.code.push(code);
        func_idx
    }

    /// Add a memory
    pub fn add_memory(&mut self, memory: Memory) -> u32 {
        let idx = self.memories.len() as u32;
        self.memories.push(memory);
        idx
    }

    /// Add a global variable
    pub fn add_global(&mut self, global: Global) -> u32 {
        let idx = self.globals.len() as u32;
        self.globals.push(global);
        idx
    }

    /// Add an export
    pub fn add_export(&mut self, export: Export) {
        self.exports.push(export);
    }

    /// Encode the module to binary format
    pub fn encode(&self) -> Vec<u8> {
        let mut encoder = WasmEncoder::new();

        // Magic number
        encoder.write_bytes(&[0x00, 0x61, 0x73, 0x6D]); // "\0asm"

        // Version
        encoder.write_bytes(&[0x01, 0x00, 0x00, 0x00]); // version 1

        // Type section
        if !self.types.is_empty() {
            encoder.write_section(1, |enc| {
                enc.write_unsigned(self.types.len() as u32);
                for func_type in &self.types {
                    enc.write_byte(0x60); // func type tag
                    enc.write_unsigned(func_type.params.len() as u32);
                    for param in &func_type.params {
                        enc.write_byte(param.to_byte());
                    }
                    enc.write_unsigned(func_type.results.len() as u32);
                    for result in &func_type.results {
                        enc.write_byte(result.to_byte());
                    }
                }
            });
        }

        // Import section
        if !self.imports.is_empty() {
            encoder.write_section(2, |enc| {
                enc.write_unsigned(self.imports.len() as u32);
                for import in &self.imports {
                    enc.write_string(&import.module);
                    enc.write_string(&import.name);
                    match &import.kind {
                        ImportKind::Func(type_idx) => {
                            enc.write_byte(0x00); // func import
                            enc.write_unsigned(*type_idx);
                        }
                        ImportKind::Memory(limits) => {
                            enc.write_byte(0x02); // memory import
                            encode_limits(enc, limits);
                        }
                    }
                }
            });
        }

        // Function section (type indices)
        if !self.functions.is_empty() {
            encoder.write_section(3, |enc| {
                enc.write_unsigned(self.functions.len() as u32);
                for type_idx in &self.functions {
                    enc.write_unsigned(*type_idx);
                }
            });
        }

        // Memory section
        if !self.memories.is_empty() {
            encoder.write_section(5, |enc| {
                enc.write_unsigned(self.memories.len() as u32);
                for memory in &self.memories {
                    encode_limits(enc, &memory.limits);
                }
            });
        }

        // Global section
        if !self.globals.is_empty() {
            encoder.write_section(6, |enc| {
                enc.write_unsigned(self.globals.len() as u32);
                for global in &self.globals {
                    enc.write_byte(global.global_type.val_type.to_byte());
                    enc.write_byte(global.global_type.mutability.to_byte());
                    enc.write_bytes(&global.init_expr);
                }
            });
        }

        // Export section
        if !self.exports.is_empty() {
            encoder.write_section(7, |enc| {
                enc.write_unsigned(self.exports.len() as u32);
                for export in &self.exports {
                    enc.write_string(&export.name);
                    enc.write_byte(export.kind.to_byte());
                    enc.write_unsigned(export.index);
                }
            });
        }

        // Code section
        if !self.code.is_empty() {
            encoder.write_section(10, |enc| {
                enc.write_unsigned(self.code.len() as u32);
                for code in &self.code {
                    // Encode function body with size prefix
                    let mut body_encoder = WasmEncoder::new();
                    body_encoder.write_unsigned(code.locals.len() as u32);
                    for (count, val_type) in &code.locals {
                        body_encoder.write_unsigned(*count);
                        body_encoder.write_byte(val_type.to_byte());
                    }
                    body_encoder.write_bytes(&code.body);

                    // Write size + body
                    let body_data = body_encoder.finish();
                    enc.write_unsigned(body_data.len() as u32);
                    enc.write_bytes(&body_data);
                }
            });
        }

        encoder.finish()
    }
}

impl Default for WasmModule {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to encode memory limits
fn encode_limits(enc: &mut WasmEncoder, limits: &Limits) {
    match limits.max {
        None => {
            enc.write_byte(0x00); // no maximum
            enc.write_unsigned(limits.min);
        }
        Some(max) => {
            enc.write_byte(0x01); // has maximum
            enc.write_unsigned(limits.min);
            enc.write_unsigned(max);
        }
    }
}
