use loxvm::chunk::{Chunk, OpCode};
use loxvm::vm::VM;


fn main() {
    
    let mut vm = VM::new();
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::OP_CONSTANT(constant), 123);

    let constant = chunk.add_constant(3.4);
    chunk.write_chunk(OpCode::OP_CONSTANT(constant), 123);

    chunk.write_chunk(OpCode::OP_ADD, 123);

    chunk.write_chunk(OpCode::OP_NEGATE, 123);
    chunk.write_chunk(OpCode::OP_RETURN, 123);
    
    
    vm.interpret(&chunk);

}
