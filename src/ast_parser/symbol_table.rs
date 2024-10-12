
use pest::Stack;
use std::collections::HashMap;

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
        return SymbolTable{
            stack : HashMap::new(),
            functions : HashMap::new(),
            anim_stack : Stack::new()
        }
    }
}