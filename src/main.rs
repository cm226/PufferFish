use pest::Parser;
use pest_derive::Parser;

mod asm_generator;
mod asm_compiler;
mod ast_parser;

#[derive(Parser)]
#[grammar = "language.pest"]
pub struct PuffParser;

fn main() {

    let mut generator = asm_generator::code_generator::Generator::new();
    let unparsed_file = std::fs::read_to_string("program.puff").expect("cannot read file");

    let file = PuffParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails
    
    ast_parser::ast_parser::generate_from_ast(file, &mut generator); 

    let output = generator.generate();
    
    match asm_compiler::compile_asm(&output) {
        Err(e) => { 
            println!("Failed to compile asm!: {}", e);
        },
        Ok(_)=> { println!("Program Compiled Successfully")}
    }

}
