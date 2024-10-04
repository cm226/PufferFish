use pest::iterators::Pair;

use crate::{
asm_generator::{
    asm_helpers::{gen_animation, INSTRUCTION}, code_generator::{
        Generator, Instruction
    }},
Rule};
use crate::ast_parser::scope::Scope;

mod ast_functions;
mod scope;
mod ast_util;

fn op_to_instr(op : &str, gen: &mut Generator) { 
    match op {
        "+" => {
            gen.add_inst(Instruction::from(INSTRUCTION::ADD, ["edx","eax"]));
        },
        "-" => {
            gen.add_inst(Instruction::from(INSTRUCTION::SUB, ["edx","eax"]));
        },
        "*" => {
            gen.add_inst(Instruction::from(INSTRUCTION::MUL,["edx"]));
            gen.add_inst(Instruction::from(INSTRUCTION::MOV,["edx","eax"]));
        }
        _ => unreachable!()
    }
}

// Note Recursive, 
// Contract, result is always put into edx
fn parse_expression(expression : Pair<Rule>, gen : &mut Generator, scope: &mut Scope)-> Result<(), String> {
    for exp in expression.into_inner() {
        match exp.as_rule() {
            Rule::value => {
                parse_expression(exp, gen, scope)?;
            },
            Rule::number => {
                gen.add_inst(Instruction::from(INSTRUCTION::MOV,["edx", exp.as_str()]));
            },
            Rule::varname =>{
                let offset = scope.stack.get(exp.as_str())
                    .ok_or(format!("Variable {} is not defined", exp.as_str()))?;

                gen.add_inst(Instruction::from(INSTRUCTION::MOV,["rdx",&format!("[rbp-{}]",offset)]));
            }
            Rule::complex_expression => {
                let mut complex = exp.into_inner();
                
                // parser is not greedy so for expressions like 1+2+3
                // we hold on to the first (in this case 1), calculate 2+3 (the second part)
                // then evaluate the whole. 1 + (2+3)
                let first = complex.next().unwrap();
                let op = complex.next().unwrap().as_str();
                parse_expression(complex.next().unwrap(), gen, scope)?;
                gen.add_inst(Instruction::from(INSTRUCTION::MOV,["eax","edx"]));
                parse_expression(first, gen, scope)?;
                op_to_instr(op, gen);
            },
            Rule::function => {
                parse_fn_call(exp, gen, scope)?;
            },
            _ => unreachable!()
        }
    }
    Ok(())
}

fn parse_assignment(assignment : Pair<Rule>, gen : &mut Generator, scope: &mut Scope) -> Result<(), String>{
    let mut ex_it = assignment.into_inner();
    let var_name = ex_it.next().unwrap();
    let value = ex_it.next().unwrap();        

    parse_expression(value, gen, scope)?;

    let offset = scope.stack.get(var_name.as_str())
        .ok_or(format!("Variable {} is not defined", var_name))?;

    gen.add_inst(Instruction::from(INSTRUCTION::MOV,[&format!("[rbp-{}]",offset), "rdx"]));    

    Ok(())
}

fn parse_fn_call(fn_call : Pair<Rule>, gen : &mut Generator, scope: &mut Scope) -> Result<(), String> { 

    let mut fn_it = fn_call.into_inner();
    let fn_name = fn_it.next().unwrap();
    let fn_expression = fn_it.next().unwrap();

    match fn_name.as_str() {
        "print" => {
            parse_expression(fn_expression, gen, scope)?;
            // move the thing to print into eax thats where we will print from
            gen.add_inst(Instruction::from(INSTRUCTION::MOV,["eax", "edx"]));
            gen.add_inst(Instruction::from(INSTRUCTION::CALL,["print_fn"]));
        },
        "anim" => {
            scope.function.get(fn_expression.as_str()).ok_or(format!("Function {} is not defined", fn_expression.as_str()))?;
            scope.anim_stack.push(String::from(fn_expression.as_str()));
        },
        _ => {
            parse_expression(fn_expression, gen, scope)?;
            scope.function.get(fn_name.as_str()).ok_or(format!("Function: {} is not defined", fn_name.as_str()))?;
            gen.add_inst(Instruction::from(INSTRUCTION::MOV,["rdi", "rdx"])); // X64 calling convention, first param is in rdi
            gen.add_inst(Instruction::from(INSTRUCTION::CALL,[fn_name.as_str()]));

        }
    }

    Ok(())
}

fn parse_declaration(dec : Pair<Rule>, gen : &mut Generator, scope: &mut Scope) -> Result<(), String>{

    let mut fn_it = dec.into_inner();
    let varname = fn_it.next().unwrap();
    let expression = fn_it.next().unwrap();

    parse_expression(expression, gen, scope)?; // result not in edx

    ast_util::push_var_to_stack(varname.as_str(), scope);
    gen.add_inst(Instruction::from(INSTRUCTION::PUSH,["rdx"]));

    Ok(())
}


fn parse_line(line: Pair<Rule>, generator : &mut Generator, scope: &mut Scope) -> Result<(), String>{
    for expression in line.into_inner() {
        match expression.as_rule() {
            Rule::assignment => {
                parse_assignment(expression, generator, scope)?;
            },
            Rule::var_declaration => {
                parse_declaration(expression, generator, scope)?;
            },
            Rule::fn_declaration => {
                ast_functions::parse_fn_declaration(expression, generator, scope)?;
            },
            Rule::expression => {
                parse_expression(expression, generator, scope)?;
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}

pub fn generate_from_ast(ast : Pair<Rule>, generator : &mut Generator) -> Result<(), String> {
    let mut scope = Scope::new();

    // setup the stack frame
    generator.add_inst(Instruction::from(INSTRUCTION::PUSH, ["rbp"]));
    generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rbp", "rsp"]));

    for line in ast.into_inner() {
        match line.as_rule() {
            Rule::EOI => Ok(()),
            Rule::line => parse_line(line, generator, &mut scope),
            _ => unreachable!(),
        }?;
    }

    gen_animation(generator, scope.anim_stack); 
    
    // cleanup the stack frame
    generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rsp", "rbp"]));
    generator.add_inst(Instruction::from(INSTRUCTION::POP, ["rbp"]));


    Ok(())
}