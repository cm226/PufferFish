use crate::asm_generator::{
    asm_helpers::INSTRUCTION,
    code_generator::Instruction
};
use crate::ast_parser::symbol_table::{SymbolTable, FunctionType};
use crate::asm_generator::code_generator::{Generator,Label};
use crate::ast_types::{self};
use crate::ast_parser::ast_util::push_var_to_stack;

pub fn parse_fn_declaration(fn_decleration : &ast_types::FnDeclaration, generator : &mut Generator, scope: &mut SymbolTable) -> Result<(), String> { 

    let fn_type = scope.functions.get(fn_decleration.name.value.as_str())
        .ok_or(format!("Function not defined {}", fn_decleration.name.value))?;
    match fn_type{
        FunctionType::XY => 
            parse_xy_decleration(fn_decleration, generator),
        FunctionType::NORMAL =>
            parse_normal_fn_declaration(fn_decleration, generator)
    }
}

fn parse_xy_decleration(fn_dec : &ast_types::FnDeclaration, gen : &mut Generator) -> Result<(), String> {
    let mut fn_generator = Generator::new();
    let mut fn_scope : SymbolTable = SymbolTable::new();

    fn_generator.add_label(Label::from(fn_dec.name.value.as_str()));
    start_stack_frame(&mut fn_generator);
    push_args_to_stack(fn_dec, &mut fn_generator, &mut fn_scope);

    push_var_to_stack("x", &mut fn_scope);
    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rdi", "0"]));
    fn_generator.add_inst(Instruction::from(INSTRUCTION::PUSH, ["rdi"]));

    push_var_to_stack("y", &mut fn_scope);
    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rdi", "0"]));
    fn_generator.add_inst(Instruction::from(INSTRUCTION::PUSH, ["rdi"]));

    generate_code_for_fn_body(fn_dec, &mut fn_generator, &mut fn_scope)?;

    let x_offset = fn_scope.stack.get("x")
        .ok_or(format!("Variable {} is not defined", "x"))?;

    let y_offset = fn_scope.stack.get("y")
        .ok_or(format!("Variable {} is not defined", "y"))?;
    
    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rdi", &format!("[rbp-{}]",x_offset)]));
    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rsi", &format!("[rbp-{}]",y_offset)]));
    fn_generator.add_inst(Instruction::from(INSTRUCTION::CALL, ["draw_shape"]));

    end_stack_frame(&mut fn_generator);
    gen.append(&mut fn_generator);

    Ok(())
}

fn parse_normal_fn_declaration(fn_dec : &ast_types::FnDeclaration, gen : &mut Generator) -> Result<(), String> { 

    let mut fn_generator = Generator::new();
    let mut fn_scope : SymbolTable = SymbolTable::new();

    fn_generator.add_label(Label::from(fn_dec.name.value.as_str()));
    start_stack_frame(&mut fn_generator);
    push_args_to_stack(fn_dec, &mut fn_generator, &mut fn_scope);
    generate_code_for_fn_body(fn_dec, &mut fn_generator, &mut fn_scope)?;
    end_stack_frame(&mut fn_generator);
    gen.append(&mut fn_generator);

    Ok(())

}

fn start_stack_frame(
    fn_generator : &mut Generator 
) {
    fn_generator.add_inst(Instruction::from(INSTRUCTION::PUSH, ["rbp"]));
    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rbp", "rsp"]));
}

fn end_stack_frame(
    fn_generator : &mut Generator 
) {
    // cleanup the stack frame
    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rsp", "rbp"]));
    fn_generator.add_inst(Instruction::from(INSTRUCTION::POP, ["rbp"]));

    fn_generator.add_inst(Instruction::from(INSTRUCTION::RET,[""]));
}

fn push_args_to_stack(
    fn_dec : &ast_types::FnDeclaration,
    fn_generator : &mut Generator,
    fn_scope: &mut SymbolTable
) { 
    // TODO curently only have support for single fn arg
    push_var_to_stack(&fn_dec.arg.value, fn_scope);
    fn_generator.add_inst(Instruction::from(INSTRUCTION::PUSH, ["rdi"]));
}

fn generate_code_for_fn_body(
    fn_dec : &ast_types::FnDeclaration, 
    fn_generator : &mut Generator, 
    fn_scope: &mut SymbolTable) -> Result<(), String> {

    for line in fn_dec.lines.iter() {
        super::parse_line(line, fn_generator, fn_scope)?;
    }

    Ok(())
} 
