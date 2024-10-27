
use pest::Stack;
use std::collections::HashMap;

use crate::asm_generator::code_generator::GLOBAL_EXTERNAL_FUNCTIONS;

    
#[derive(Clone)]
pub enum FunctionType { 
    XY,
    NORMAL
}

pub struct SymbolTable {
    pub stack: HashMap<String, usize>,
    pub functions: HashMap<String, FunctionType>,
    pub anim_stack: Stack<String>
}

impl SymbolTable { 
    pub fn new() -> SymbolTable{
        let normal_fn = std::iter::repeat(FunctionType::NORMAL);

        return SymbolTable{
            stack : HashMap::new(),
            functions : HashMap::from_iter(
                GLOBAL_EXTERNAL_FUNCTIONS
                .iter()
                .cloned()
                .map(|f|{String::from(f)})
                .zip(normal_fn)),
            anim_stack : Stack::new()
        }
    }
}