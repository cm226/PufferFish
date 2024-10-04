use pest::iterators::Pair;
use crate::asm_generator::{
    asm_helpers::INSTRUCTION,
    code_generator::Instruction
};
use crate::ast_parser::scope::Scope;
use crate::asm_generator::code_generator::{Generator,Label};
use crate::Rule;
use crate::ast_parser::ast_util::push_var_to_stack;

pub fn parse_fn_declaration(fn_dec : Pair<Rule>, gen : &mut Generator, scope: &mut Scope) -> Result<(), String> { 

    let mut fn_generator = Generator::new();
    let mut fn_scope : Scope = Scope::new();

    let mut fn_it = fn_dec.into_inner();
    let fn_name = fn_it.next().unwrap();

    fn_generator.add_label(Label::from(fn_name.as_str()));

    // setup the stack frame
    fn_generator.add_inst(Instruction::from(INSTRUCTION::PUSH, ["rbp"]));
    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rbp", "rsp"]));

    // This language only allows a single param.... so ima just yeet this corner off, nothing to see here
    let param_name = fn_it.next().unwrap();
    push_var_to_stack(param_name.as_str(), &mut fn_scope);
    fn_generator.add_inst(Instruction::from(INSTRUCTION::PUSH, ["rdi"]));

    push_var_to_stack("x", &mut fn_scope);
    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rdi", "0"]));
    fn_generator.add_inst(Instruction::from(INSTRUCTION::PUSH, ["rdi"]));

    push_var_to_stack("y", &mut fn_scope);
    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rdi", "0"]));
    fn_generator.add_inst(Instruction::from(INSTRUCTION::PUSH, ["rdi"]));

    while let Some(line) = fn_it.next() {
        super::parse_line(line, &mut fn_generator, &mut fn_scope)?;
    }

    let x_offset = fn_scope.stack.get("x")
        .ok_or(format!("Variable {} is not defined", "x"))?;

    let y_offset = fn_scope.stack.get("y")
        .ok_or(format!("Variable {} is not defined", "y"))?;
    
    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rdi", &format!("[rbp-{}]",x_offset)]));
    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rsi", &format!("[rbp-{}]",y_offset)]));
    fn_generator.add_inst(Instruction::from(INSTRUCTION::CALL, ["draw_shape"]));

    // cleanup the stack frame
    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rsp", "rbp"]));
    fn_generator.add_inst(Instruction::from(INSTRUCTION::POP, ["rbp"]));

    fn_generator.add_inst(Instruction::from(INSTRUCTION::RET,[""]));

    scope.function.insert(String::from(fn_name.as_str()));
    gen.append(&mut fn_generator);

    Ok(())

}