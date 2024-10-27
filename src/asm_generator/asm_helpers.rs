use super::code_generator::{Generator, Instruction, Label};

use pest::Stack;
use strum_macros::Display;

#[allow(dead_code)]
#[derive(Display)]
pub enum INSTRUCTION { 
  MOVQ,
  MOVD,
  MOVSD,
  MOVAPD,
  MOV,
  INT,
  ADD,
  SUB,
  ADDSD,
  SUBSD,
  MULSD, 
  DIVSD,
  PUSH,
  POP,
  CMP,
  JNZ,
  RET,
  CALL,
  INC,
  LOOP,
  DEC,
  SYSCALL,
  CVTSD2SI,
  CVTSI2SD
}

pub fn gen_animation(gen: &mut Generator, mut anim_stack: Stack<String>) {

  if anim_stack.len() == 0 {
    return;
  }
  let loop_count = 500;

  gen.add_inst(Instruction::from(INSTRUCTION::CALL, ["create_window"]));
  gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["rcx", &loop_count.to_string()]));
  gen.add_inst(Instruction::from(INSTRUCTION::PUSH, ["rcx"]));
  gen.add_label(Label::from("anim_loop"));
 
  while let Some(anim_fn) =  anim_stack.pop() {
    gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["rcx", "[rsp]"]));
    gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["rdi", &loop_count.to_string()]));
    gen.add_inst(Instruction::from(INSTRUCTION::SUB, ["rdi", "rcx"]));

    gen.add_inst(Instruction::from(INSTRUCTION::CVTSI2SD, ["xmm0", "rdi"]));
    gen.add_inst(Instruction::from(INSTRUCTION::MOVQ, ["rdi", "xmm0"]));

    gen.add_inst(Instruction::from(INSTRUCTION::CALL, [anim_fn]));
  }

  // allign the stack to 16-bit address, required when calling c functions
  gen.add_inst(Instruction::from(INSTRUCTION::PUSH, ["rbp"]));
  gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["rbp", "rsp"]));
  gen.add_inst(Instruction::from(INSTRUCTION::CALL, ["blit"]));
  gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["rsp", "rbp"]));
  gen.add_inst(Instruction::from(INSTRUCTION::POP, ["rbp"]));

  gen.add_inst(Instruction::from(INSTRUCTION::POP, ["rcx"]));
  gen.add_inst(Instruction::from(INSTRUCTION::DEC, ["rcx"]));
  gen.add_inst(Instruction::from(INSTRUCTION::PUSH, ["rcx"]));

  gen.add_inst(Instruction::from(INSTRUCTION::LOOP, ["anim_loop"]));
  
  gen.add_inst(Instruction::from(INSTRUCTION::POP, ["rcx"]));
  gen.add_inst(Instruction::from(INSTRUCTION::CALL, ["destroy_window"]));
  
}
