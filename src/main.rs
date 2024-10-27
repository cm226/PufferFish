use clap::{arg, command};

use crate::from_pest::FromPest;

mod ast_types;
mod asm_generator;
mod asm_compiler;
mod ast_parser;

extern crate pest_derive;
extern crate from_pest;
#[macro_use]
extern crate pest_ast;
extern crate pest;

fn main() {

    let default_out = String::from("output");
    let matches = command!() // requires `cargo` feature
        .arg(arg!(
            -v --verbose ... "Enable verbose compiler output"
        ).action(clap::ArgAction::SetTrue))
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ).action(clap::ArgAction::SetTrue))
        .arg(arg!(-o --output <OUTPUT> "Optional Output file [default = output]"))
        .arg(arg!([inputFile] "Program source"))
        .get_matches();


    let generate_debug_info = matches.get_flag("debug");

    let input_file_path = matches.get_one::<String>("inputFile")
        .expect("input file is required");
    
    let output_file_path = matches.get_one::<String>("output")
        .or(Some(&default_out)).unwrap();
    
    
    let unparsed_file = std::fs::read_to_string(input_file_path).expect("cannot read file");
    
    let mut generator = asm_generator::code_generator::Generator::new();
    
    use pest::Parser;

    let mut file = ast_types::PuffParser::parse(ast_types::Rule::file, &unparsed_file).unwrap();
    let syntax_tree = ast_types::File::from_pest(&mut file).expect("infallible");

    if matches.get_flag("verbose"){
        println!("{:?}", syntax_tree);
    }

    if let Err(e) = ast_parser::generate_from_ast(syntax_tree, &mut generator) {
        eprintln!("Compilation Error : {}", e);
        std::process::exit(1);
    }

    let output = generator.generate();
    
   match asm_compiler::compile_asm(&output, generate_debug_info, output_file_path) {
        Err(e) => { 
            eprintln!("Failed to compile asm!: {}", e);
            std::process::exit(1);
        },
        Ok(_)=> { println!("Program Compiled Successfully")}
    }

}
