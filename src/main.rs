use pest::Parser;
use pest_derive::Parser;
use clap::{arg, command};

mod asm_generator;
mod asm_compiler;
mod ast_parser;

#[derive(Parser)]
#[grammar = "language.pest"]
pub struct PuffParser;

fn main() {

    let default_out = String::from("output");
    let matches = command!() // requires `cargo` feature
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ).action(clap::ArgAction::SetTrue))
        .arg(arg!(-o --output <OUTPUT> "Optinal Output file [default = output]"))
        .arg(arg!([inputFile] "Program source"))
        .get_matches();


    let generate_debug_info = matches.get_flag("debug");

    let input_file_path = matches.get_one::<String>("inputFile")
        .expect("input file is required");
    
    let output_file_path = matches.get_one::<String>("output")
        .or(Some(&default_out)).unwrap();
    
    
    let unparsed_file = std::fs::read_to_string(input_file_path).expect("cannot read file");
    
    let mut generator = asm_generator::code_generator::Generator::new();
    

    let file = PuffParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails
    
    if let Err(e) = ast_parser::ast_parser::generate_from_ast(file, &mut generator) {
        eprintln!("Compilation Error : {}", e);
        std::process::exit(1);
    }

    let output = generator.generate();
    
    match asm_compiler::compile_asm(&output, generate_debug_info, output_file_path) {
        Err(e) => { 
            println!("Failed to compile asm!: {}", e);
        },
        Ok(_)=> { println!("Program Compiled Successfully")}
    }

}
