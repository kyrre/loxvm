use crate::chunk::{Chunk, OpCode};
use crate::debug::{disassemble_instruction, print_value};
use crate::value::Value;
use crate::compiler::Compiler;


#[allow(non_camel_case_types)]
pub enum InterpretResult {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR,
}

pub struct VM<'a> {
    chunk: Option<&'a Chunk>,
    stack: Vec<Value>,
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        VM {
            chunk: None,
            stack: Vec::new(),
        }
    }
    pub fn interpret(source: &str) -> InterpretResult {
    //    compile(source);

       InterpretResult::INTERPRET_OK
    }

    // pub fn interpret(&mut self, chunk: &'a Chunk) -> InterpretResult {
        // self.chunk = Some(chunk);
        // self.run()
    // }

    pub fn run(&mut self) -> InterpretResult {
        use InterpretResult::*;
        use OpCode::*;

        if let Some(chunk) = self.chunk {
            for (ip, instruction) in chunk.code.iter().enumerate() {
                disassemble_instruction(chunk, ip);

                match instruction {
                    OP_RETURN => {
                        print_value(&self.pop());
                        return INTERPRET_OK;
                    }
                    OP_CONSTANT(constant) => {
                        self.push(chunk.constant_at(*constant));
                    }
                    OP_ADD => self.BINARY_OP('+'),
                    OP_SUBTRACT => self.BINARY_OP('-'),
                    OP_MULTIPLY => self.BINARY_OP('*'),
                    OP_DIVIDE => self.BINARY_OP('*'),
                    OP_NEGATE => {
                        if let Value::Number(num) = self.pop() {
                            self.push(Value::Number(-num));
                        }
                    }
                }
            }
        }

        INTERPRET_RUNTIME_ERROR
    }

    #[allow(non_camel_case_types, non_snake_case)]
    fn BINARY_OP(&mut self, op: char) {
        if let (Value::Number(a), Value::Number(b)) = (self.pop(), self.pop()) {
            let result = match op {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => a / b,
                _ => panic!("UNSUPPORTED OPERATOR")
            };
            self.push(Value::Number(result));
        } else {
            panic!("RUNTIME ERROR");
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}
