use std::fs;

use code_generator::Instruction;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

mod code_generator;

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
    println!("whole assignement is {}", assignment.as_str());
    
    let mut ex_it = assignment.into_inner();
    let var_name = ex_it.next().unwrap();
    let value = ex_it.next().unwrap();        

    println!("var {} assigned to :", var_name);
    parse_expression(value);
}


fn main() {

    let mut generator = code_generator::Generator::new();

    generator.add_inst(Instruction{
        instruction:"mov".to_string(),
        args:vec!["edx".to_string(), "len".to_string()]
    });

    generator.generate();


    // let unparsed_file = fs::read_to_string("program.puff").expect("cannot read file");

    // let file = CSVParser::parse(Rule::file, &unparsed_file)
    //     .expect("unsuccessful parse") // unwrap the parse result
    //     .next().unwrap(); // get and unwrap the `file` rule; never fails
    
    //     for line in file.into_inner() {
    //         match line.as_rule() {
    //             Rule::EOI => (),
    //             Rule::line => {
    //                 for expression in line.into_inner() {
    //                     match expression.as_rule() {
    //                         Rule::assignment => {
    //                             parse_assignment(expression);
    //                         },
    //                         Rule::declaration => {
    //                             println!("Declaration");
    //                         },
    //                         _ => unreachable!(),
    //                     }
    //                 }
                    
    //             },
    //             _ => unreachable!(),
    //         }
    //     }
}
