use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::value::Value;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    
    println!("== {} == ", name);
    
    let mut offset = 0;
    while offset < chunk.count() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    
    
    if offset > 0 && (chunk.line_at(offset) == chunk.line_at(offset - 1)) {
        print!("   | ");
    }
    else {
        print!("{:04} ", chunk.line_at(offset) )
    }

    match chunk.at(offset) {
        OpCode::OP_RETURN => simple_instruction("OP_RETURN", offset),
        OpCode::OP_CONSTANT(constant) => constant_instruction("OP_CONSTANT", chunk, *constant, offset),
        OpCode::OP_NEGATE => simple_instruction("OP_NEGATE", offset),
        OpCode::OP_ADD => simple_instruction("OP_ADD", offset),
        OpCode::OP_SUBTRACT => simple_instruction("OP_SUBTRACT", offset),
        OpCode::OP_DIVIDE => simple_instruction("OP_DIVIDE", offset),
        _ => {
            println!("Unknown opcode");
            offset + 1
        }
    }
}

pub fn simple_instruction(name: &str, offset: usize) -> usize{
    println!("{}", name);
    offset + 1
}

pub fn constant_instruction(name: &str, chunk: &Chunk, constant: usize, offset: usize) -> usize {
    print!("{} {:4} ", name, constant);
    print_value(&chunk.constant_at(constant));
    offset + 1 
}

pub fn print_value(value: &Value) {
   if let Value::Number(constant)  = value {
    println!("{}", constant);
   }
}