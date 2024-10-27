use crate::asm_generator::asm_helpers::INSTRUCTION;
use crate::ast_parser::symbol_table::SymbolTable;
use crate::asm_generator::code_generator::{Generator,Instruction};

pub fn push_var_to_stack(name: &str,scope: &mut SymbolTable ) {

    // I must be missing something here?
    // (scope.len() * 8) i would have thought would be the address of the next thing that will be added to the stack
    // so the +4 shouldn't be needed. But it looks like it is needed. 
    // So when rsb == rsp (theres nothing on the stack) when you add the first thing, you need to use the address rsb + 8? 
    // so thats stored at esb? 
    scope.stack.insert(String::from(name), (scope.stack.len() * 8)+8);
}


pub fn with_alligned_stack(symbol_table : &SymbolTable, gen : &mut Generator, f: &dyn Fn(&mut Generator)->()){
  allign_stack(symbol_table, gen);
  f(gen);
  unallign_stack(symbol_table, gen);
}

fn allign_stack(scope : &SymbolTable, gen : &mut Generator) {
  
    let is_aligned = (scope.stack.len()%2) == 0;
    if !is_aligned{
        gen.add_inst(Instruction::from(INSTRUCTION::SUB, ["rsp","8"])); // allign the stack to 16-byte
    } 
}

fn unallign_stack(scope : &SymbolTable, gen : &mut Generator) {

    let is_aligned = (scope.stack.len()%2) == 0;
    if !is_aligned{
        gen.add_inst(Instruction::from(INSTRUCTION::ADD, ["rsp","8"]));
    } 
}