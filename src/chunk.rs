use crate::value::Value;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Copy, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    OP_CONSTANT(usize),
    OP_NEGATE,
    OP_ADD,
    OP_SUBTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
    OP_RETURN,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
    constants: Vec<Value>,
    lines: Vec<usize>
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new()
        }
    }
    
    pub fn count(&self) -> usize {
        self.code.len()
    }
    
    pub fn write_chunk(&mut self, byte: OpCode, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }
    
    pub fn add_constant(&mut self, constant: f64) -> usize {
        self.constants.push(Value::Number(constant));
        self.constants.len() - 1
    }
    
    
    pub fn free(&mut self) {
        self.code.clear();
        self.constants.clear();
        self.lines.clear();
    }
    
    pub fn at(&self, offset: usize) -> &OpCode {
        &self.code[offset]
    }
    
    pub fn constant_at(&self, index: usize) -> Value {
        self.constants[index]
    }
    
    pub fn line_at(&self, index: usize) -> usize {
        self.lines[index]
    }
}