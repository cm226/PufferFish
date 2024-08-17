use std::collections::HashMap;

use pest::iterators::Pair;

use crate::{
asm_generator::{
    self, asm_helpers::INSTRUCTION, code_generator::{
        Generator, Instruction, Label
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
fn parse_expression(expression : Pair<Rule>, gen : &mut Generator, stack_vars: &mut HashMap<String, usize>)-> Result<(), String> {
    for exp in expression.into_inner() {
        match exp.as_rule() {
            Rule::value => {
                parse_expression(exp, gen, stack_vars)?;
            },
            Rule::number => {
                gen.add_inst(Instruction::from(INSTRUCTION::MOV,["edx", exp.as_str()]));
            },
            Rule::varname =>{
                let offset = stack_vars.get(exp.as_str())
                    .ok_or(format!("Variable {} is not defined", exp.as_str()))?;

                gen.add_inst(Instruction::from(INSTRUCTION::MOV,["edx",&format!("[ebp-{}]",offset)]));
            }
            Rule::complex_expression => {
                let mut complex = exp.into_inner();
                parse_expression(complex.next().unwrap(), gen, stack_vars)?;
                gen.add_inst(Instruction::from(INSTRUCTION::MOV,["eax","edx"]));
                let op = complex.next().unwrap().as_str();
                parse_expression(complex.next().unwrap(), gen, stack_vars)?;
                op_to_instr(op, gen);
            },
            Rule::function => {
                parse_fn_call(exp, gen, stack_vars)?;
            },
            _ => unreachable!()
        }
    }
    Ok(())
}

fn parse_assignment(assignment : Pair<Rule>, gen : &mut Generator, stack_vars: &mut HashMap<String, usize>) -> Result<(), String>{
    let mut ex_it = assignment.into_inner();
    let var_name = ex_it.next().unwrap();
    let value = ex_it.next().unwrap();        

    parse_expression(value, gen, stack_vars)?;

    let offset = stack_vars.get(var_name.as_str())
        .ok_or(format!("Variable {} is not defined", var_name))?;

    gen.add_inst(Instruction::from(INSTRUCTION::MOV,[&format!("[ebp-{}]",offset), "edx"]));    

    Ok(())
}

fn parse_fn_call(fn_call : Pair<Rule>, gen : &mut Generator, stack_vars: &mut HashMap<String, usize>) -> Result<(), String> { 

    let mut fn_it = fn_call.into_inner();
    let fn_name = fn_it.next().unwrap();
    let fn_expression = fn_it.next().unwrap();

    parse_expression(fn_expression, gen, stack_vars)?;

    match fn_name.as_str() {
        "print" => {
            // move the thing to print into eax thats where we will print from
            gen.add_inst(Instruction::from(INSTRUCTION::MOV,["eax", "edx"]));
            gen.add_inst(Instruction::from(INSTRUCTION::CALL,["print_fn"]));
        },
        _ => {
            gen.add_inst(Instruction::from(INSTRUCTION::CALL,[fn_name.as_str()]));
        }
    }

    Ok(())
}

fn parse_declaration(dec : Pair<Rule>, gen : &mut Generator, stack_vars: &mut HashMap<String, usize>) -> Result<(), String>{

    let mut fn_it = dec.into_inner();
    let varname = fn_it.next().unwrap();
    let expression = fn_it.next().unwrap();

    parse_expression(expression, gen, stack_vars)?; // result not in edx

    // I must be missing something here?
    // (stack_vars.len() * 4) i would have thought would be the address of the next thing that will be added to the stack
    // so the +4 shouldn't be needed. But it looks like it is needed. 
    // So when esb == esp (theres nothing on the stack) when you add the first thing, you need to use the address esb + 4? 
    // so thats stored at esb? 
    stack_vars.insert(varname.as_str().to_string(), (stack_vars.len() * 4)+4);
    gen.add_inst(Instruction::from(INSTRUCTION::PUSH,["edx"]));

    Ok(())
}

fn parse_fn_declaration(fn_dec : Pair<Rule>, gen : &mut Generator) -> Result<(), String> { 

    let mut fn_generator = asm_generator::code_generator::Generator::new();
    let mut stack_vars : HashMap<String, usize> = HashMap::new();

    let mut fn_it = fn_dec.into_inner();
    let fn_name = fn_it.next().unwrap();

    fn_generator.add_label(Label::from(fn_name.as_str()));

    // setup the stack frame
    fn_generator.add_inst(Instruction::from(INSTRUCTION::PUSH, ["ebp"]));
    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["ebp", "esp"]));

    // This language only allows a single param.... so ima just yeet this corner off, nothing to see here
    let param_name = fn_it.next().unwrap();
    stack_vars.insert(String::from(param_name.as_str()), 0);
    fn_generator.add_inst(Instruction::from(INSTRUCTION::PUSH, ["edx"]));

    while let Some(line) = fn_it.next() {
        parse_line(line, &mut fn_generator, &mut stack_vars)?;
    }

    // cleanup the stack frame
    fn_generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["esp", "ebp"]));
    fn_generator.add_inst(Instruction::from(INSTRUCTION::POP, ["ebp"]));

    fn_generator.add_inst(Instruction::from(INSTRUCTION::RET,[""]));

    gen.append(&mut fn_generator);

    Ok(())

}

fn parse_line(line: Pair<Rule>, generator : &mut Generator, stack_vars: &mut HashMap<String, usize>) -> Result<(), String>{
    for expression in line.into_inner() {
        match expression.as_rule() {
            Rule::assignment => {
                parse_assignment(expression, generator, stack_vars)?;
            },
            Rule::var_declaration => {
                parse_declaration(expression, generator, stack_vars)?;
            },
            Rule::fn_declaration => {
                parse_fn_declaration(expression, generator)?;
            },
            Rule::expression => {
                parse_expression(expression, generator, stack_vars)?;
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}

pub fn generate_from_ast(ast : Pair<Rule>, generator : &mut Generator) -> Result<(), String> {
    let mut stack_vars : HashMap<String, usize> = HashMap::new();

    // setup the stack frame
    generator.add_inst(Instruction::from(INSTRUCTION::PUSH, ["ebp"]));
    generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["ebp", "esp"]));

    for line in ast.into_inner() {
        match line.as_rule() {
            Rule::EOI => Ok(()),
            Rule::line => parse_line(line, generator, &mut stack_vars),
            _ => unreachable!(),
        }?;
    }

    // cleanup the stack frame
    generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["esp", "ebp"]));
    generator.add_inst(Instruction::from(INSTRUCTION::POP, ["ebp"]));

    Ok(())
}
