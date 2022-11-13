use loxvm::chunk::{Chunk, OpCode};
use loxvm::vm::VM;
use std::process;

use std::io::{self, Write};

struct Lox<'a> {
    vm: VM<'a>,
    chunk: Chunk,
}

impl<'a> Lox<'a> {
    fn new() -> Self {
        Lox {
            vm: VM::new(),
            chunk: Chunk::new(),
        }
    }

    fn repl(&self) {
        let mut line = String::new();
        let input = std::io::stdin();

        loop {
            print!("> ");
            io::stdout().flush().expect("Failed to flush stdout");

            if let Ok(n) = input.read_line(&mut line) {
                if n == 0 {
                    break;
                }

                // self.vm.

                // self.vm.interpret(line);


            }
        }
    }

    fn run_file(&self, filename: &str) {}

    fn test(&'a mut self) {
        let constant = self.chunk.add_constant(1.2);
        self.chunk.write_chunk(OpCode::OP_CONSTANT(constant), 123);

        let constant = self.chunk.add_constant(3.4);
        self.chunk.write_chunk(OpCode::OP_CONSTANT(constant), 123);

        self.chunk.write_chunk(OpCode::OP_ADD, 123);

        self.chunk.write_chunk(OpCode::OP_NEGATE, 123);
        self.chunk.write_chunk(OpCode::OP_RETURN, 123);

        //self.vm.interpret(&self.chunk);
    }
}

fn main() {
    let mut lox = Lox::new();

    let argc = std::env::args().len();
    if argc == 1 {
        lox.repl();
    } else if argc == 2 {
    } else {
        lox.test();
        eprintln!("Usage loxvm [path]");
        std::process::exit(64);
    }
}
