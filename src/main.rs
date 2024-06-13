use asm_generator::{asm_helpers::INSTRUCTION, code_generator::{Data, Generator, Instruction}};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

mod asm_generator;
mod asm_compiler;

#[derive(Parser)]
#[grammar = "language.pest"]
pub struct CSVParser;

fn op_to_instr(op : &str, gen: &mut Generator) { 
    match op {
        "+" => {
            gen.add_inst(Instruction{
                instruction:INSTRUCTION::ADD,
                args:vec!["edx".to_string(),"eax".to_string()]
            });
        },
        "*" => {
            gen.add_inst(Instruction{
                instruction:INSTRUCTION::MUL,
                args:vec!["edx".to_string()]
            });

            gen.add_inst(Instruction{
                instruction:INSTRUCTION::MOV,
                args:vec!["edx".to_string(), "eax".to_string()]
            });
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
                gen.add_inst(Instruction{
                    instruction:INSTRUCTION::MOV,
                    args:vec!["edx".to_string(),String::from(exp.as_str())]
                });
            },
            Rule::varname =>{
                gen.add_inst(Instruction{
                    instruction:INSTRUCTION::MOV,
                    args:vec!["edx".to_string(),format!("[{}]",exp.as_str())]
                });
            }
            Rule::complex_expression => {
                let mut complex = exp.into_inner();
                parse_expression(complex.next().unwrap(), gen);

                gen.add_inst(Instruction{
                    instruction:INSTRUCTION::MOV,
                    args:vec!["eax".to_string(),"edx".to_string()]
                });

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

    gen.add_inst(Instruction{
        instruction:INSTRUCTION::MOV,
        args:vec![format!("[{}]",var_name.as_str()), "edx".to_string()]
    });
}

fn parse_fn_call(fn_call : Pair<Rule>, gen : &mut Generator) { 

    let mut fn_it = fn_call.into_inner();
    let fn_name = fn_it.next().unwrap();
    let fn_expression = fn_it.next().unwrap();

    parse_expression(fn_expression, gen);

    match fn_name.as_str() {
        "print" => asm_generator::asm_helpers::gen_std_out("edx", 1, gen),
        _ => ()
    }
}

fn parse_declaration(dec : Pair<Rule>, gen : &mut Generator) {

    let mut fn_it = dec.into_inner();
    let varname = fn_it.next().unwrap();
    let expression = fn_it.next().unwrap();

    parse_expression(expression, gen); // result not in edx

    gen.add_bss(Data{
        name: varname.as_str().to_string(),
        kind: "resb".to_string(),
        args: vec!["1".to_string()]
    });

    gen.add_inst(Instruction{
        instruction:INSTRUCTION::MOV,
        args: vec![format!("[{}]",varname.as_str().to_string()), "edx".to_string()]
    })
}

fn main() {

    let mut generator = asm_generator::code_generator::Generator::new();

    generator.add_bss(Data { 
        name: "print".to_string(),
        kind: "resb".to_string(),
        args: vec!["1".to_string()] });

    
    let unparsed_file = std::fs::read_to_string("program.puff").expect("cannot read file");

    let file = CSVParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails
    
        for line in file.into_inner() {
            match line.as_rule() {
                Rule::EOI => (),
                Rule::line => {
                    for expression in line.into_inner() {
                        match expression.as_rule() {
                            Rule::assignment => {
                                parse_assignment(expression, &mut generator);
                            },
                            Rule::declaration => {
                                parse_declaration(expression, &mut generator);
                            },
                            Rule::function => {
                                parse_fn_call(expression, &mut generator);
                            },
                            Rule::expression => {
                                parse_expression(expression, &mut generator);
                            }
                            _ => unreachable!(),
                        }
                    }
                    
                },
                _ => unreachable!(),
            }
        }

        let output = generator.generate();
        
        match asm_compiler::compile_asm(&output) {
            Err(e) => { 
                println!("Failed to compile asm!: {}", e);
            },
            Ok(_)=> { println!("Program Compiled Successfully")}
        }

}
