use pest::iterators::Pair;

use crate::{
asm_generator::{
    self,
    asm_helpers::INSTRUCTION,
    code_generator::{
        Data, Generator, Instruction
    }},
Rule};


fn op_to_instr(op : &str, gen: &mut Generator) { 
    match op {
        "+" => {
            gen.add_inst(Instruction::from(INSTRUCTION::ADD, ["edx","eax"]));
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
fn parse_expression(expression : Pair<Rule>, gen : &mut Generator) {
    for exp in expression.into_inner() {
        match exp.as_rule() {
            Rule::value => {
                parse_expression(exp, gen);
            },
            Rule::number => {
                gen.add_inst(Instruction::from(INSTRUCTION::MOV,["edx", exp.as_str()]));
            },
            Rule::varname =>{
                gen.add_inst(Instruction::from(INSTRUCTION::MOV,["edx",&format!("[{}]",exp.as_str())]));
            }
            Rule::complex_expression => {
                let mut complex = exp.into_inner();
                parse_expression(complex.next().unwrap(), gen);
                gen.add_inst(Instruction::from(INSTRUCTION::MOV,["eax","edx"]));
                let op = complex.next().unwrap().as_str();
                parse_expression(complex.next().unwrap(), gen);
                op_to_instr(op, gen);
            },
            _ => unreachable!()
        }
    }
}

fn parse_assignment(assignment : Pair<Rule>, gen : &mut Generator) {
   
    let mut ex_it = assignment.into_inner();
    let var_name = ex_it.next().unwrap();
    let value = ex_it.next().unwrap();        

    parse_expression(value, gen);
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,[&format!("[{}]",var_name.as_str()), "edx"]));
}

fn parse_fn_call(fn_call : Pair<Rule>, gen : &mut Generator) { 

    let mut fn_it = fn_call.into_inner();
    let fn_name = fn_it.next().unwrap();
    let fn_expression = fn_it.next().unwrap();

    parse_expression(fn_expression, gen);

    match fn_name.as_str() {
        "print" => {
            // move the thing to print into eax thats where we will print from
            gen.add_inst(Instruction::from(INSTRUCTION::MOV,["eax", "edx"]));
            gen.add_inst(Instruction::from(INSTRUCTION::CALL,["print_fn"]));
        },
        _ => ()
    }
}

fn parse_declaration(dec : Pair<Rule>, gen : &mut Generator) {

    let mut fn_it = dec.into_inner();
    let varname = fn_it.next().unwrap();
    let expression = fn_it.next().unwrap();

    parse_expression(expression, gen); // result not in edx

    gen.add_bss(Data::from(varname.as_str(),"RESD", ["1"]));
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,[&format!("[{}]",varname.as_str().to_string()), "edx"]));
}

pub fn generate_from_ast(ast : Pair<Rule>, generator : &mut Generator) { 
    for line in ast.into_inner() {
        match line.as_rule() {
            Rule::EOI => (),
            Rule::line => {
                for expression in line.into_inner() {
                    match expression.as_rule() {
                        Rule::assignment => {
                            parse_assignment(expression, generator);
                        },
                        Rule::declaration => {
                            parse_declaration(expression, generator);
                        },
                        Rule::function => {
                            parse_fn_call(expression, generator);
                        },
                        Rule::expression => {
                            parse_expression(expression, generator);
                        }
                        _ => unreachable!(),
                    }
                }
                
            },
            _ => unreachable!(),
        }
    }
}
