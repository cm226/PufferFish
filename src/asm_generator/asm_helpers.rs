use crate::{ast_parser::{ast_util::{pop_stack_to_reg, push_reg_to_stack}, symbol_table::SymbolTable}, errors::compiler_errors::CompilerErrors};

use super::{calling_convention_imp::{call_with, Args}, code_generator::{Generator, Instruction, Label}};

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

pub fn gen_animation(gen: &mut Generator, scope: &mut SymbolTable) -> Result<(), CompilerErrors>{


  if scope.anim_stack.len() == 0 {
    return Ok(());
  }

  let loop_count = 500;

  call_with("create_window", [].iter(), gen, scope)?;

  scope.anim_stack.snapshot();
  while let Some(anim_fn) = scope.anim_stack.pop() { 
    call_with(&anim_fn.shape, [Args::Int(1)].iter(), gen, scope)?;  
    call_with("loadImageTex", [Args::IntReg("rdi")].iter(), gen, scope)?;
    push_reg_to_stack(&scope.anim_stack.len().to_string(), &mut scope.hidden_stack, gen, "rax");
  }
  scope.anim_stack.restore();
 
  gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["rcx", &loop_count.to_string()]));
  push_reg_to_stack("anim_loop_counter", &mut scope.hidden_stack, gen, "rcx");
  gen.add_label(Label::from("anim_loop"));
 
  while let Some(anim_fn) =  scope.anim_stack.pop() {
    gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["rcx", "[rsp]"]));
    gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["rdi", &loop_count.to_string()]));
    gen.add_inst(Instruction::from(INSTRUCTION::SUB, ["rdi", "rcx"]));

    gen.add_inst(Instruction::from(INSTRUCTION::CVTSI2SD, ["xmm0", "rdi"]));
    gen.add_inst(Instruction::from(INSTRUCTION::MOVQ, ["rdi", "xmm0"]));

    call_with(&anim_fn.xy,[Args::StrPtr("rdi")].iter() , gen, scope)?;

    let shape_name = scope.anim_stack.len().to_string();
    let offset = scope.hidden_stack.get_stack_address(&shape_name)?;
    call_with("draw_shape", [Args::StrPtr("rdi"), Args::StrPtr("rsi"), Args::IntStack(*offset)].iter(), gen,scope)?;
  }

  call_with("blit", [].iter(), gen, scope)?;
  pop_stack_to_reg("anim_loop_counter", "rcx", &mut scope.hidden_stack, gen)?;
  gen.add_inst(Instruction::from(INSTRUCTION::DEC, ["rcx"]));
  push_reg_to_stack("anim_loop_counter", &mut scope.hidden_stack, gen, "rcx");

  gen.add_inst(Instruction::from(INSTRUCTION::JNZ, ["anim_loop"]));
  
  pop_stack_to_reg("anim_loop_counter", "rcx", &mut scope.hidden_stack, gen)?;

  call_with("destroy_window", [].iter(), gen, scope)?;
  Ok(())
  
}
