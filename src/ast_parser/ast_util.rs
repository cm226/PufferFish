use crate::asm_generator::asm_helpers::INSTRUCTION;
use crate::ast_parser::symbol_table::SymbolTable;
use crate::asm_generator::code_generator::{Generator,Instruction};
use crate::errors::compiler_errors::CompilerErrors;

use super::symbol_table::VarStack;

pub fn push_reg_to_stack(
    name: &str,
    var_stack: &mut VarStack,
    gen : &mut Generator,
    reg : &str) {
    var_stack.add(String::from(name));
    gen.add_inst(Instruction::from(INSTRUCTION::PUSH, [reg]));
}

pub fn pop_stack_to_reg(
    name: &str,
    reg: &str, 
    var_stack : &mut VarStack, 
    gen : &mut Generator
) -> Result<(), CompilerErrors>{
    // TODO can we get the address of the name, and check it matches what we are about to pop?
    // Maybe we can check at least that is has the highest offset? 
    var_stack.get_stack_address(name)?;
    gen.add_inst(Instruction::from(INSTRUCTION::POP, [reg]));
    Ok(())
}

pub fn with_aligned_stack(symbol_table : &SymbolTable, gen : &mut Generator, f: &dyn Fn(&mut Generator)->()){
  align_stack(symbol_table, gen);
  f(gen);
  unalign_stack(symbol_table, gen);
}

fn align_stack(scope : &SymbolTable, gen : &mut Generator) {
  
    let is_aligned = scope.get_stack_allocs()%2 == 0;
    if !is_aligned{
        gen.add_inst(Instruction::from(INSTRUCTION::SUB, ["rsp","8"])); // align the stack to 16-byte
    } 
}

fn unalign_stack(scope : &SymbolTable, gen : &mut Generator) {

    let is_aligned = scope.get_stack_allocs()%2 == 0;
    if !is_aligned{
        gen.add_inst(Instruction::from(INSTRUCTION::ADD, ["rsp","8"]));
    } 
}