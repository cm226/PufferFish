use asm_generator::code_generator::{Data, Generator, Instruction};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

mod asm_generator;
mod asm_compiler;

#[derive(Parser)]
#[grammar = "language.pest"]
pub struct CSVParser;

fn parse_expression(expression : Pair<Rule>) { 
    println!("whole exp {:?}", expression.as_str());
    for exp in expression.into_inner() {
        match exp.as_rule() {
            Rule::number => println!("num {}", exp.as_str()),
            Rule::complex_expression => {
                let mut complex = exp.into_inner();
                let first = complex.next().unwrap().as_str();
                let op = complex.next().unwrap().as_str();
                let second = complex.next().unwrap().as_str();
                println!("assigned to {} {} {}", first, op, second);
            },
            _ => ()
        }
    }
}

fn parse_assignment(assignment : Pair<Rule>) {
    println!("whole assignment is {}", assignment.as_str());
    
    let mut ex_it = assignment.into_inner();
    let var_name = ex_it.next().unwrap();
    let value = ex_it.next().unwrap();        

    println!("var {} assigned to :", var_name);
    parse_expression(value);
}

fn parse_fn_call(fn_call : Pair<Rule>, gen : &mut Generator) { 
    println!("whole function call is {}", fn_call.as_str());

    let mut fn_it = fn_call.into_inner();
    let fn_name = fn_it.next().unwrap();
    let fn_expression = fn_it.next().unwrap();
    println!("whole function name {}", fn_name.as_str());
    println!("whole function expression {}", fn_expression.as_str());

    gen.add_inst(Instruction{
        instruction:"mov".to_string(),
        args:vec!["edx".to_string(),format!("'{}'",fn_expression.as_str())]
    });

    gen.add_inst(Instruction{
        instruction:"mov".to_string(),
        args:vec!["[print]".to_string(),"edx".to_string()]
    });

    match fn_name.as_str() {
        "print" => asm_generator::asm_helpers::gen_std_out("print", 1, gen),
        _ => ()
    }

}


fn main() {

    let mut generator = asm_generator::code_generator::Generator::new();

    generator.add_bss(Data { 
        name: "print".to_string(),
        kind: "resb".to_string(),
        args: vec!["1".to_string()] });

    // generator.add_inst(Instruction{
    //     instruction:"mov".to_string(),
    //     args:vec!["edx".to_string(), "len".to_string()]
    // });

    // generator.add_inst(Instruction{
    //     instruction:"mov".to_string(),
    //     args:vec!["ecx".to_string(), "msg".to_string()]
    // });

    // generator.add_inst(Instruction{
    //     instruction:"mov".to_string(),
    //     args:vec!["ebx".to_string(), "1".to_string()]
    // });

    // generator.add_inst(Instruction{
    //     instruction:"mov".to_string(),
    //     args:vec!["eax".to_string(), "4".to_string()]
    // });

    // generator.add_inst(Instruction{
    //     instruction:"int".to_string(),
    //     args:vec!["0x80".to_string()]
    // });

    // generator.add_data(Data { 
    //     name: "msg".to_string(),
    //     kind: "db".to_string(),
    //     args: vec!["'Hello, world!'".to_owned(), "0xa".to_owned()]
    // });

    // generator.add_data(Data { 
    //     name: "len".to_string(),
    //     kind: "equ".to_string(),
    //     args: vec!["$ - msg".to_string()]
    // });
    
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
                                parse_assignment(expression);
                            },
                            Rule::declaration => {
                                println!("Declaration");
                            },
                            Rule::function => {
                                parse_fn_call(expression, &mut generator);
                            }
                            _ => unreachable!(),
                        }
                    }
                    
                },
                _ => unreachable!(),
            }
        }

        let output = generator.generate();
        asm_compiler::compile_asm(&output);
}
