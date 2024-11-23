use crate::asm_generator::calling_convention_imp::{self};
use crate::asm_generator::{
    asm_helpers::INSTRUCTION,
    code_generator::Instruction
};
use crate::ast_parser::symbol_table::{SymbolTable, FunctionType};
use crate::asm_generator::code_generator::{Generator,Label};
use crate::ast_types::{self};
use crate::ast_parser::ast_util::push_reg_to_stack;
use crate::errors::compiler_errors::CompilerErrors;

pub fn parse_fn_declaration(fn_decleration : &ast_types::FnDeclaration, generator : &mut Generator, scope: &mut SymbolTable) -> Result<(), CompilerErrors> { 

    let fn_type = scope.functions.get(fn_decleration.name.value.as_str())
        .ok_or(CompilerErrors::MissingFunction(fn_decleration.name.value.clone()))?;
    match fn_type{
        FunctionType::XY => 
            parse_xy_decleration(fn_decleration, generator),
        FunctionType::NORMAL =>
            parse_normal_fn_declaration(fn_decleration, generator),
        FunctionType::SHAPE => 
            parse_shape_function(fn_decleration, generator)
    }
}

fn parse_shape_function(fn_dec : &ast_types::FnDeclaration, gen : &mut Generator) -> Result<(), CompilerErrors> {
    let mut fn_generator = Generator::new();
    let mut fn_scope : SymbolTable = SymbolTable::new();

    fn_generator.add_label(Label::from(fn_dec.name.value.as_str()));
    start_stack_frame(&mut fn_generator, &mut fn_scope);
    push_args_to_stack(fn_dec, &mut fn_generator, &mut fn_scope)?;

    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rdi", "0"]));
    push_reg_to_stack("tex", &mut fn_scope.visible_stack, &mut fn_generator, "rdi");

    generate_code_for_fn_body(fn_dec, &mut fn_generator, &mut fn_scope)?;

    let tex_offset = fn_scope.visible_stack.get_stack_address("tex")?;
    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rdi", &format!("[rbp-{}]",tex_offset)]));
    
    end_stack_frame(&mut fn_generator, &mut fn_scope);
    gen.append(&mut fn_generator);

    Ok(())
}

fn parse_xy_decleration(fn_dec : &ast_types::FnDeclaration, gen : &mut Generator) -> Result<(), CompilerErrors> {
    let mut fn_generator = Generator::new();
    let mut fn_scope : SymbolTable = SymbolTable::new();

    fn_generator.add_label(Label::from(fn_dec.name.value.as_str()));
    start_stack_frame(&mut fn_generator, &mut fn_scope);
    push_args_to_stack(fn_dec, &mut fn_generator, &mut fn_scope)?;

    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rdi", "0"]));
    push_reg_to_stack("x", &mut fn_scope.visible_stack, &mut fn_generator, "rdi");

    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rdi", "0"]));
    push_reg_to_stack("y", &mut fn_scope.visible_stack, &mut fn_generator, "rdi");

    generate_code_for_fn_body(fn_dec, &mut fn_generator, &mut fn_scope)?;

    let x_offset = fn_scope.visible_stack.get_stack_address("x")?;
    let y_offset = fn_scope.visible_stack.get_stack_address("y")?;

    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOVQ, ["xmm0", &format!("[rbp-{}]",x_offset)]));
    fn_generator.add_inst(Instruction::from(INSTRUCTION::CVTSD2SI, ["rdi", "xmm0"]));

    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOVQ, ["xmm0", &format!("[rbp-{}]",y_offset)]));
    fn_generator.add_inst(Instruction::from(INSTRUCTION::CVTSD2SI, ["rsi", "xmm0"]));
    
    end_stack_frame(&mut fn_generator, &mut fn_scope);
    gen.append(&mut fn_generator);

    Ok(())
}

fn parse_normal_fn_declaration(fn_dec : &ast_types::FnDeclaration, gen : &mut Generator) -> Result<(), CompilerErrors> { 

    let mut fn_generator = Generator::new();
    let mut fn_scope : SymbolTable = SymbolTable::new();

    fn_generator.add_label(Label::from(fn_dec.name.value.as_str()));
    start_stack_frame(&mut fn_generator, &mut fn_scope);
    push_args_to_stack(fn_dec, &mut fn_generator, &mut fn_scope)?;
    generate_code_for_fn_body(fn_dec, &mut fn_generator, &mut fn_scope)?;
    end_stack_frame(&mut fn_generator, &mut fn_scope);
    gen.append(&mut fn_generator);

    Ok(())

}

pub fn start_stack_frame(
    fn_generator : &mut Generator,
    _scope: &mut SymbolTable
) {
    fn_generator.add_inst(Instruction::from(INSTRUCTION::PUSH, ["rbp"]));
    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rbp", "rsp"]));
}

pub fn end_stack_frame(
    fn_generator : &mut Generator,
    _scope: &mut SymbolTable
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
) -> Result<(), CompilerErrors> { 

    let varnames : Vec<&str> = fn_dec.args.iter().map(|f|{f.value.as_str()}).collect();
    calling_convention_imp::push_values_from_arg_reg_into_stack(varnames.iter(), fn_generator, &mut fn_scope.visible_stack)?;
    Ok(())
}

fn generate_code_for_fn_body(
    fn_dec : &ast_types::FnDeclaration, 
    fn_generator : &mut Generator, 
    fn_scope: &mut SymbolTable) -> Result<(), CompilerErrors> {

    for line in fn_dec.lines.iter() {
        super::parse_line(line, fn_generator, fn_scope)?;
    }

    Ok(())
} 
