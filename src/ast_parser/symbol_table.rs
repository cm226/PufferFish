
use pest::Stack;
use std::collections::HashMap;

use crate::asm_generator::code_generator::GLOBAL_EXTERNAL_FUNCTIONS;

    
#[derive(Clone)]
pub enum FunctionType { 
    XY,
    NORMAL,
    SHAPE
}

#[derive(Clone)]
pub struct AnimPair { 
    pub xy : String,
    pub shape : String
}

pub struct SymbolTable {
    pub stack: HashMap<String, usize>,
    pub anonymous_stack_alloc : usize,
    pub functions: HashMap<String, FunctionType>,
    pub anim_stack: Stack<AnimPair>
}

impl SymbolTable { 
    pub fn new() -> SymbolTable{
        let normal_fn = std::iter::repeat(FunctionType::NORMAL);

        return SymbolTable{
            stack : HashMap::new(),
            anonymous_stack_alloc : 0,
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