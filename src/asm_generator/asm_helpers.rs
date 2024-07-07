
use super::code_generator::{Generator, Instruction, Label};

use strum_macros::Display;

#[derive(Display)]
pub enum INSTRUCTION { 
  MOV,
  INT,
  ADD,
  MUL, 
  DIV,
  PUSH,
  CMP,
  JNZ,
  RET,
  CALL
}

pub fn gen_std_out_fn(gen : &mut Generator) { 

    gen.add_label(Label::from("print_fn"));
    
    // setup some registers we will need
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["ebx", "10"])); // divisor
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["ecx", "0"])); // character counter

    // covert base 2 to base 10 and push to stack
    gen.add_label(Label::from("convert_loop"));
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["edx", "0"]));
    gen.add_inst(Instruction::from(INSTRUCTION::DIV,["ebx"]));
    gen.add_inst(Instruction::from(INSTRUCTION::ADD,["edx", "'0'"]));
    gen.add_inst(Instruction::from(INSTRUCTION::ADD,["ecx", "2"]));
    gen.add_inst(Instruction::from(INSTRUCTION::PUSH,["dx"]));
    // Are we done yet?
    gen.add_inst(Instruction::from(INSTRUCTION::CMP,["eax", "0"]));
    gen.add_inst(Instruction::from(INSTRUCTION::JNZ,["convert_loop"]));

    // std write the stack
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["edx", "ecx"]));
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["ecx", "esp"]));
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["ebx", "1"]));
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["eax", "4"]));
    gen.add_inst(Instruction::from(INSTRUCTION::INT,["0x80"]));

    gen.add_inst(Instruction::from(INSTRUCTION::ADD,["esp", "edx"]));
    gen.add_inst(Instruction::from(INSTRUCTION::RET,[""]));

}