

use ast_util::with_alligned_stack;

use crate::{
asm_generator::{
    self, asm_helpers::{gen_animation, INSTRUCTION}, calling_convention_imp, code_generator::{
        Generator, Instruction
    }},
ast_types::{self}};

mod ast_functions;
pub mod symbol_table;
pub mod ast_util;

fn op_to_instr(op : &ast_types::Operator, gen: &mut Generator, lhs : &str, rhs: &str) { 
    match op.value.as_str() {
        "+" => {
            gen.add_inst(Instruction::from(INSTRUCTION::ADDSD, [lhs,rhs]));
        },
        "-" => {
            gen.add_inst(Instruction::from(INSTRUCTION::SUBSD, [lhs,rhs]));
        },
        "*" => {
            gen.add_inst(Instruction::from(INSTRUCTION::MULSD,[lhs, rhs]));
        }
        _ => unreachable!()
    }
}

fn parse_value(value : &ast_types::Value, gen: &mut Generator, scope: &mut symbol_table::SymbolTable) -> Result<(), String>{
    match value { 
        ast_types::Value::Number(num) =>{
            let literal = format!(" __float64__({})",num.value);
            gen.add_inst(Instruction::from(INSTRUCTION::MOV,["rdx",literal.as_str()]));
            gen.add_inst(Instruction::from(INSTRUCTION::MOVQ, ["xmm0", "rdx"]));
            Ok(())
        }
        ast_types::Value::Varname(varname) => {
            let offset = scope.stack.get(&varname.value)
                .ok_or(format!("Variable {} is not defined", varname.value))?;

            gen.add_inst(Instruction::from(INSTRUCTION::MOVQ,["xmm0",&format!("[rbp-{}]",offset)]));
            Ok(())
        }
    }
}
// Note Recursive, 
// Contract, result is always put into edx
fn parse_expression(expression : &ast_types::Expression, gen : &mut Generator, scope: &mut symbol_table::SymbolTable)-> Result<(), String> {
        match expression {
            ast_types::Expression::Value(value) => {
                parse_value(value, gen, scope)
            },
            ast_types::Expression::Complex(complex) => {
                // parser is not greedy so for expressions like 1+2+3
                // we hold on to the first (in this case 1), calculate 2+3 (the second part)
                // then evaluate the whole. 1 + (2+3)
                parse_expression(&complex.expression[0], gen, scope)?;
                gen.add_inst(Instruction::from(INSTRUCTION::MOVSD,["xmm1","xmm0"]));

                parse_value(&complex.value, gen, scope)?;
                op_to_instr(&complex.opperator, gen,"xmm0", "xmm1");
                Ok(())
            },
             ast_types::Expression::Function(fnc) => {
                parse_fn_call(fnc, gen, scope)
            }
        }
}

fn parse_assignment(assignment : &ast_types::Assignment, gen : &mut Generator, scope: &mut symbol_table::SymbolTable) -> Result<(), String>{

    parse_expression(&assignment.expression, gen, scope)?;

    let offset = scope.stack.get(&assignment.varname.value)
        .ok_or(format!("Variable {} is not defined", assignment.varname.value))?;

    gen.add_inst(Instruction::from(INSTRUCTION::MOVQ, ["rdx", "xmm0"]));
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,[&format!("[rbp-{}]",offset), "rdx"]));  

    Ok(())
}

fn parse_fn_call(fn_call : &ast_types::Function, gen : &mut Generator, scope: &mut symbol_table::SymbolTable) -> Result<(), String> {  
    use asm_generator::calling_convention_imp::Args;
    match fn_call.name.value.as_str() {
        "print" => {
            parse_expression(&fn_call.arg[0], gen, scope)?;
            asm_generator::calling_convention_imp::call_with(
                "printf", 
                [Args::StrPtr("print_fmt_str"),Args::FloatReg("xmm0")],
                gen, scope)?
        },
        "anim" => {
            match &fn_call.arg[0] {
               ast_types::Expression::Value(fn_name) => {
                    match fn_name {
                        ast_types::Value::Varname(fn_name) => {
                            scope.functions.get(&fn_name.value).ok_or(format!("Function {} is not defined", fn_name.value))?;
                            scope.anim_stack.push(String::from(&fn_name.value));
                        }
                        _ => unreachable!()
                    } 
               },
               _ => unreachable!()
            }
        },
        _ => {
            parse_expression(&fn_call.arg[0], gen, scope)?;
            scope.functions.get(&fn_call.name.value).ok_or(format!("Function: {} is not defined", fn_call.name.value))?;
            use asm_generator::calling_convention_imp::Args;
            calling_convention_imp::call_with(
                &fn_call.name.value,
                [Args::FloatReg("XMM0")],
                gen, scope)?;
            // put ret in RDX

            gen.add_inst(Instruction::from(INSTRUCTION::MOVQ, ["rdx", "xmm0"]));
        }
    }

    Ok(())
}

fn parse_declaration(dec : &ast_types::VarDeclaration, gen : &mut Generator, scope: &mut symbol_table::SymbolTable) -> Result<(), String>{
    parse_expression(&dec.value, gen, scope)?; // result in xmm0

    ast_util::push_var_to_stack(&dec.name.value, scope);

    gen.add_inst(Instruction::from(INSTRUCTION::MOVQ, ["rdx", "xmm0"]));
    gen.add_inst(Instruction::from(INSTRUCTION::PUSH,["rdx"]));

    Ok(())
}


fn parse_line(line: &ast_types::Line, generator : &mut Generator, scope: &mut symbol_table::SymbolTable) -> Result<(), String>{
    match line {
        ast_types::Line::Assignment(assignment) => {
            parse_assignment(assignment, generator, scope)
        },
        ast_types::Line::Decleration(decleration) => {
            parse_declaration(decleration, generator, scope)
        },
        ast_types::Line::FnDeclaration(fn_decleration) => {
            ast_functions::parse_fn_declaration(fn_decleration, generator, scope)
        },
        ast_types::Line::Expression(expression) => {
            parse_expression(expression, generator, scope)
        }
    }
}

pub fn populate_symbols(ast : &ast_types::File, symbols : &mut symbol_table::SymbolTable) {

    for line in ast.lines.iter(){
        match line {
            ast_types::Line::Expression(exp) => {
                match exp {
                    ast_types::Expression::Function(call) => {
                        match call.name.value.as_str() { 
                            "anim" => {
                                match &call.arg[0] {
                                    ast_types::Expression::Value(fn_name) => { 
                                        match fn_name {
                                            ast_types::Value::Varname(varname) => { 
                                                symbols.functions.insert(String::from(varname.value.as_str()), symbol_table::FunctionType::XY);
                                                ()
                                            }
                                            _ => unreachable!()
                                        }
                                    },
                                    _ => unreachable!()
                                }
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
            },
            ast_types::Line::FnDeclaration(fn_decl) => {
                // Normal fn type by default, overriden by specialty functions
                if !symbols.functions.contains_key(fn_decl.name.value.as_str()) {
                    symbols.functions.insert(String::from(fn_decl.name.value.as_str()),
                     symbol_table::FunctionType::NORMAL);
                }
            },
            _ => ()
        }
    }
}

pub fn generate_from_ast(ast : ast_types::File, generator : &mut Generator) -> Result<(), String> {
    let mut scope = symbol_table::SymbolTable::new();
    populate_symbols(&ast, &mut scope);

    // setup the stack frame
    generator.add_inst(Instruction::from(INSTRUCTION::PUSH, ["rbp"]));
    generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rbp", "rsp"]));

    for line in ast.lines {
        parse_line(&line, generator, &mut scope)?;
    }

    // flush the output before we exit
    with_alligned_stack(&scope, generator, &|gen| {
        gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["rdi","0"]));
        gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["rax","0"]));
        gen.add_inst(Instruction::from(INSTRUCTION::CALL, ["fflush"]));
    });

    gen_animation(generator, scope.anim_stack); 
    // cleanup the stack frame
    generator.add_inst(Instruction::from(INSTRUCTION::MOV, ["rsp", "rbp"]));
    generator.add_inst(Instruction::from(INSTRUCTION::POP, ["rbp"]));


    Ok(())
}