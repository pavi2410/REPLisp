/// JVM Class File Format for Java 8
/// Based on the Java Virtual Machine Specification (Java SE 8 Edition)

use std::collections::HashMap;

/// Access flags for classes, methods, and fields
#[derive(Debug, Clone, Copy)]
pub struct AccessFlags(pub u16);

impl AccessFlags {
    pub const ACC_PUBLIC: u16 = 0x0001;
    pub const ACC_PRIVATE: u16 = 0x0002;
    pub const ACC_PROTECTED: u16 = 0x0004;
    pub const ACC_STATIC: u16 = 0x0008;
    pub const ACC_FINAL: u16 = 0x0010;
    pub const ACC_SUPER: u16 = 0x0020;
    pub const ACC_SYNCHRONIZED: u16 = 0x0020;
    pub const ACC_VOLATILE: u16 = 0x0040;
    pub const ACC_BRIDGE: u16 = 0x0040;
    pub const ACC_TRANSIENT: u16 = 0x0080;
    pub const ACC_VARARGS: u16 = 0x0080;
    pub const ACC_NATIVE: u16 = 0x0100;
    pub const ACC_INTERFACE: u16 = 0x0200;
    pub const ACC_ABSTRACT: u16 = 0x0400;
    pub const ACC_STRICT: u16 = 0x0800;
    pub const ACC_SYNTHETIC: u16 = 0x1000;
    pub const ACC_ANNOTATION: u16 = 0x2000;
    pub const ACC_ENUM: u16 = 0x4000;

    pub fn new(flags: u16) -> Self {
        AccessFlags(flags)
    }

    pub fn public_class() -> Self {
        AccessFlags(Self::ACC_PUBLIC | Self::ACC_SUPER)
    }

    pub fn public_static() -> Self {
        AccessFlags(Self::ACC_PUBLIC | Self::ACC_STATIC)
    }

    pub fn public_method() -> Self {
        AccessFlags(Self::ACC_PUBLIC)
    }
}

/// Constant pool entries
#[derive(Debug, Clone)]
pub enum ConstantPoolEntry {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(u16),
    String(u16),
    Fieldref(u16, u16),
    Methodref(u16, u16),
    InterfaceMethodref(u16, u16),
    NameAndType(u16, u16),
}

impl ConstantPoolEntry {
    fn tag(&self) -> u8 {
        match self {
            ConstantPoolEntry::Utf8(_) => 1,
            ConstantPoolEntry::Integer(_) => 3,
            ConstantPoolEntry::Float(_) => 4,
            ConstantPoolEntry::Long(_) => 5,
            ConstantPoolEntry::Double(_) => 6,
            ConstantPoolEntry::Class(_) => 7,
            ConstantPoolEntry::String(_) => 8,
            ConstantPoolEntry::Fieldref(_, _) => 9,
            ConstantPoolEntry::Methodref(_, _) => 10,
            ConstantPoolEntry::InterfaceMethodref(_, _) => 11,
            ConstantPoolEntry::NameAndType(_, _) => 12,
        }
    }

    fn write(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.tag());

        match self {
            ConstantPoolEntry::Utf8(s) => {
                let bytes = s.as_bytes();
                buffer.push((bytes.len() >> 8) as u8);
                buffer.push(bytes.len() as u8);
                buffer.extend_from_slice(bytes);
            }
            ConstantPoolEntry::Integer(val) => {
                buffer.extend_from_slice(&val.to_be_bytes());
            }
            ConstantPoolEntry::Float(val) => {
                buffer.extend_from_slice(&val.to_be_bytes());
            }
            ConstantPoolEntry::Long(val) => {
                buffer.extend_from_slice(&val.to_be_bytes());
            }
            ConstantPoolEntry::Double(val) => {
                buffer.extend_from_slice(&val.to_be_bytes());
            }
            ConstantPoolEntry::Class(name_index) => {
                buffer.push((name_index >> 8) as u8);
                buffer.push(*name_index as u8);
            }
            ConstantPoolEntry::String(string_index) => {
                buffer.push((string_index >> 8) as u8);
                buffer.push(*string_index as u8);
            }
            ConstantPoolEntry::Fieldref(class_index, nat_index) |
            ConstantPoolEntry::Methodref(class_index, nat_index) |
            ConstantPoolEntry::InterfaceMethodref(class_index, nat_index) => {
                buffer.push((class_index >> 8) as u8);
                buffer.push(*class_index as u8);
                buffer.push((nat_index >> 8) as u8);
                buffer.push(*nat_index as u8);
            }
            ConstantPoolEntry::NameAndType(name_index, descriptor_index) => {
                buffer.push((name_index >> 8) as u8);
                buffer.push(*name_index as u8);
                buffer.push((descriptor_index >> 8) as u8);
                buffer.push(*descriptor_index as u8);
            }
        }
    }

    /// Returns true if this entry takes two slots in the constant pool (long/double)
    pub fn is_double_width(&self) -> bool {
        matches!(self, ConstantPoolEntry::Long(_) | ConstantPoolEntry::Double(_))
    }
}

/// Constant pool manager
pub struct ConstantPool {
    entries: Vec<ConstantPoolEntry>,
    utf8_cache: HashMap<String, u16>,
    class_cache: HashMap<u16, u16>,
    string_cache: HashMap<u16, u16>,
    methodref_cache: HashMap<(u16, u16), u16>,
    nat_cache: HashMap<(u16, u16), u16>,
}

impl ConstantPool {
    pub fn new() -> Self {
        ConstantPool {
            entries: Vec::new(),
            utf8_cache: HashMap::new(),
            class_cache: HashMap::new(),
            string_cache: HashMap::new(),
            methodref_cache: HashMap::new(),
            nat_cache: HashMap::new(),
        }
    }

    fn add_entry(&mut self, entry: ConstantPoolEntry) -> u16 {
        // Calculate the actual index, accounting for double-width entries
        let mut index = 1; // Constant pool indices start at 1
        for e in &self.entries {
            index += 1;
            if e.is_double_width() {
                index += 1; // Long and Double take 2 slots
            }
        }
        self.entries.push(entry);
        index as u16
    }

    pub fn add_utf8(&mut self, s: String) -> u16 {
        if let Some(&index) = self.utf8_cache.get(&s) {
            return index;
        }
        let index = self.add_entry(ConstantPoolEntry::Utf8(s.clone()));
        self.utf8_cache.insert(s, index);
        index
    }

    pub fn add_class(&mut self, name: String) -> u16 {
        let name_index = self.add_utf8(name);
        if let Some(&index) = self.class_cache.get(&name_index) {
            return index;
        }
        let index = self.add_entry(ConstantPoolEntry::Class(name_index));
        self.class_cache.insert(name_index, index);
        index
    }

    pub fn add_string(&mut self, s: String) -> u16 {
        let utf8_index = self.add_utf8(s);
        if let Some(&index) = self.string_cache.get(&utf8_index) {
            return index;
        }
        let index = self.add_entry(ConstantPoolEntry::String(utf8_index));
        self.string_cache.insert(utf8_index, index);
        index
    }

    pub fn add_name_and_type(&mut self, name: String, descriptor: String) -> u16 {
        let name_index = self.add_utf8(name);
        let desc_index = self.add_utf8(descriptor);
        let key = (name_index, desc_index);
        if let Some(&index) = self.nat_cache.get(&key) {
            return index;
        }
        let index = self.add_entry(ConstantPoolEntry::NameAndType(name_index, desc_index));
        self.nat_cache.insert(key, index);
        index
    }

    pub fn add_methodref(&mut self, class: String, name: String, descriptor: String) -> u16 {
        let class_index = self.add_class(class);
        let nat_index = self.add_name_and_type(name, descriptor);
        let key = (class_index, nat_index);
        if let Some(&index) = self.methodref_cache.get(&key) {
            return index;
        }
        let index = self.add_entry(ConstantPoolEntry::Methodref(class_index, nat_index));
        self.methodref_cache.insert(key, index);
        index
    }

    pub fn add_interface_methodref(&mut self, class: String, name: String, descriptor: String) -> u16 {
        let class_index = self.add_class(class);
        let nat_index = self.add_name_and_type(name, descriptor);
        self.add_entry(ConstantPoolEntry::InterfaceMethodref(class_index, nat_index))
    }

    pub fn add_fieldref(&mut self, class: String, name: String, descriptor: String) -> u16 {
        let class_index = self.add_class(class);
        let nat_index = self.add_name_and_type(name, descriptor);
        self.add_entry(ConstantPoolEntry::Fieldref(class_index, nat_index))
    }

    pub fn add_double(&mut self, val: f64) -> u16 {
        self.add_entry(ConstantPoolEntry::Double(val))
    }

    pub fn add_integer(&mut self, val: i32) -> u16 {
        self.add_entry(ConstantPoolEntry::Integer(val))
    }

    fn write(&self, buffer: &mut Vec<u8>) {
        // Count entries (double-width entries count as 2)
        let mut count = 1; // Constant pool index starts at 1
        for entry in &self.entries {
            count += 1;
            if entry.is_double_width() {
                count += 1; // Long and Double take 2 slots
            }
        }

        buffer.push((count >> 8) as u8);
        buffer.push(count as u8);

        for entry in &self.entries {
            entry.write(buffer);
        }
    }
}

/// Method info
pub struct MethodInfo {
    pub access_flags: AccessFlags,
    pub name: String,
    pub descriptor: String,
    pub code: Vec<u8>,
    pub max_stack: u16,
    pub max_locals: u16,
}

impl MethodInfo {
    pub fn new(access_flags: AccessFlags, name: String, descriptor: String) -> Self {
        MethodInfo {
            access_flags,
            name,
            descriptor,
            code: Vec::new(),
            max_stack: 100, // Conservative default
            max_locals: 100, // Conservative default
        }
    }

    fn write(&self, buffer: &mut Vec<u8>, cp: &mut ConstantPool) {
        // Access flags
        buffer.push((self.access_flags.0 >> 8) as u8);
        buffer.push(self.access_flags.0 as u8);

        // Name index
        let name_index = cp.add_utf8(self.name.clone());
        buffer.push((name_index >> 8) as u8);
        buffer.push(name_index as u8);

        // Descriptor index
        let desc_index = cp.add_utf8(self.descriptor.clone());
        buffer.push((desc_index >> 8) as u8);
        buffer.push(desc_index as u8);

        // Attributes count
        if self.code.is_empty() {
            buffer.push(0);
            buffer.push(0);
        } else {
            buffer.push(0);
            buffer.push(1); // One Code attribute

            // Code attribute
            let code_attr_name = cp.add_utf8("Code".to_string());
            buffer.push((code_attr_name >> 8) as u8);
            buffer.push(code_attr_name as u8);

            // Attribute length (will be calculated)
            let attr_len = 12 + self.code.len() as u32; // 2(max_stack) + 2(max_locals) + 4(code_length) + code + 2(exc_table) + 2(attrs)
            buffer.extend_from_slice(&attr_len.to_be_bytes());

            // Max stack
            buffer.push((self.max_stack >> 8) as u8);
            buffer.push(self.max_stack as u8);

            // Max locals
            buffer.push((self.max_locals >> 8) as u8);
            buffer.push(self.max_locals as u8);

            // Code length
            let code_len = self.code.len() as u32;
            buffer.extend_from_slice(&code_len.to_be_bytes());

            // Code
            buffer.extend_from_slice(&self.code);

            // Exception table length
            buffer.push(0);
            buffer.push(0);

            // Attributes count
            buffer.push(0);
            buffer.push(0);
        }
    }
}

/// Field info
pub struct FieldInfo {
    pub access_flags: AccessFlags,
    pub name: String,
    pub descriptor: String,
}

impl FieldInfo {
    pub fn new(access_flags: AccessFlags, name: String, descriptor: String) -> Self {
        FieldInfo {
            access_flags,
            name,
            descriptor,
        }
    }

    fn write(&self, buffer: &mut Vec<u8>, cp: &mut ConstantPool) {
        // Access flags
        buffer.push((self.access_flags.0 >> 8) as u8);
        buffer.push(self.access_flags.0 as u8);

        // Name index
        let name_index = cp.add_utf8(self.name.clone());
        buffer.push((name_index >> 8) as u8);
        buffer.push(name_index as u8);

        // Descriptor index
        let desc_index = cp.add_utf8(self.descriptor.clone());
        buffer.push((desc_index >> 8) as u8);
        buffer.push(desc_index as u8);

        // Attributes count
        buffer.push(0);
        buffer.push(0);
    }
}

/// JVM Class File
pub struct ClassFile {
    pub version: (u16, u16), // (minor, major)
    pub constant_pool: ConstantPool,
    pub access_flags: AccessFlags,
    pub this_class: String,
    pub super_class: String,
    pub interfaces: Vec<String>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
}

impl ClassFile {
    pub fn new(class_name: String) -> Self {
        ClassFile {
            version: (0, 52), // Java 8
            constant_pool: ConstantPool::new(),
            access_flags: AccessFlags::public_class(),
            this_class: class_name,
            super_class: "java/lang/Object".to_string(),
            interfaces: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
        }
    }

    pub fn add_method(&mut self, method: MethodInfo) {
        self.methods.push(method);
    }

    pub fn add_field(&mut self, field: FieldInfo) {
        self.fields.push(field);
    }

    /// Write the class file to bytes
    pub fn write(&mut self) -> Vec<u8> {
        // First, pre-populate constant pool with all required entries
        let this_class_index = self.constant_pool.add_class(self.this_class.clone());
        let super_class_index = self.constant_pool.add_class(self.super_class.clone());

        // Pre-add interface entries
        let interface_indices: Vec<u16> = self.interfaces.iter()
            .map(|interface| self.constant_pool.add_class(interface.clone()))
            .collect();

        // Write to a temporary buffer to collect field/method constant pool entries
        let mut temp_buffer = Vec::new();

        for field in &self.fields {
            field.write(&mut temp_buffer, &mut self.constant_pool);
        }

        for method in &self.methods {
            method.write(&mut temp_buffer, &mut self.constant_pool);
        }

        // Now write the actual class file with complete constant pool
        let mut buffer = Vec::new();

        // Magic number
        buffer.extend_from_slice(&[0xCA, 0xFE, 0xBA, 0xBE]);

        // Version
        buffer.push((self.version.0 >> 8) as u8);
        buffer.push(self.version.0 as u8);
        buffer.push((self.version.1 >> 8) as u8);
        buffer.push(self.version.1 as u8);

        // Constant pool (now complete)
        self.constant_pool.write(&mut buffer);

        // Access flags
        buffer.push((self.access_flags.0 >> 8) as u8);
        buffer.push(self.access_flags.0 as u8);

        // This class
        buffer.push((this_class_index >> 8) as u8);
        buffer.push(this_class_index as u8);

        // Super class
        buffer.push((super_class_index >> 8) as u8);
        buffer.push(super_class_index as u8);

        // Interfaces
        buffer.push(0);
        buffer.push(self.interfaces.len() as u8);
        for &interface_index in &interface_indices {
            buffer.push((interface_index >> 8) as u8);
            buffer.push(interface_index as u8);
        }

        // Fields
        buffer.push(0);
        buffer.push(self.fields.len() as u8);
        // Rewrite fields with final constant pool indices
        for field in &self.fields {
            // Access flags
            buffer.push((field.access_flags.0 >> 8) as u8);
            buffer.push(field.access_flags.0 as u8);

            // Name index (already in pool)
            let name_index = self.constant_pool.add_utf8(field.name.clone());
            buffer.push((name_index >> 8) as u8);
            buffer.push(name_index as u8);

            // Descriptor index (already in pool)
            let desc_index = self.constant_pool.add_utf8(field.descriptor.clone());
            buffer.push((desc_index >> 8) as u8);
            buffer.push(desc_index as u8);

            // Attributes count
            buffer.push(0);
            buffer.push(0);
        }

        // Methods
        buffer.push(0);
        buffer.push(self.methods.len() as u8);
        // Rewrite methods with final constant pool indices
        for method in &self.methods {
            // Access flags
            buffer.push((method.access_flags.0 >> 8) as u8);
            buffer.push(method.access_flags.0 as u8);

            // Name index
            let name_index = self.constant_pool.add_utf8(method.name.clone());
            buffer.push((name_index >> 8) as u8);
            buffer.push(name_index as u8);

            // Descriptor index
            let desc_index = self.constant_pool.add_utf8(method.descriptor.clone());
            buffer.push((desc_index >> 8) as u8);
            buffer.push(desc_index as u8);

            // Attributes count
            if method.code.is_empty() {
                buffer.push(0);
                buffer.push(0);
            } else {
                buffer.push(0);
                buffer.push(1); // One Code attribute

                // Code attribute
                let code_attr_name = self.constant_pool.add_utf8("Code".to_string());
                buffer.push((code_attr_name >> 8) as u8);
                buffer.push(code_attr_name as u8);

                // Attribute length
                let attr_len = 12 + method.code.len() as u32;
                buffer.extend_from_slice(&attr_len.to_be_bytes());

                // Max stack
                buffer.push((method.max_stack >> 8) as u8);
                buffer.push(method.max_stack as u8);

                // Max locals
                buffer.push((method.max_locals >> 8) as u8);
                buffer.push(method.max_locals as u8);

                // Code length
                let code_len = method.code.len() as u32;
                buffer.extend_from_slice(&code_len.to_be_bytes());

                // Code
                buffer.extend_from_slice(&method.code);

                // Exception table length
                buffer.push(0);
                buffer.push(0);

                // Attributes count
                buffer.push(0);
                buffer.push(0);
            }
        }

        // Class attributes
        buffer.push(0);
        buffer.push(0);

        buffer
    }
}
