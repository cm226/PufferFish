
use pest::Stack;
use std::{cell::RefCell, collections::HashMap};

use crate::{asm_generator::code_generator::GLOBAL_EXTERNAL_FUNCTIONS, errors::compiler_errors::CompilerErrors};

    
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

pub struct StackAllocCounter { 
    alloc_count : usize
}

impl StackAllocCounter { 
    pub fn new() -> StackAllocCounter { 
        StackAllocCounter{
            alloc_count : 0
        }
    }

    pub fn alloc(&mut self) -> usize {
        let next = (self.alloc_count * 8)+8;
        self.alloc_count += 1;
        next
    }

    pub fn free(&mut self) {
        assert_ne!(self.alloc_count, 0);
        self.alloc_count -= 1;
    }

    pub fn get(&self) -> usize { 
        self.alloc_count
    }
}


pub struct VarStack { 
    stack : HashMap<String, usize>,
    stack_alloc : std::rc::Rc<RefCell<StackAllocCounter>>
}


impl VarStack { 
    pub fn new(stack_alloc : std::rc::Rc<RefCell<StackAllocCounter>>) -> VarStack {
        return VarStack {
            stack: HashMap::new(),
            stack_alloc : stack_alloc
        }
    }

    pub fn add(&mut self, name : String) {
        
        let position: usize = self.stack_alloc.borrow_mut().alloc();
        self.stack.insert(name, position);
    }

    pub fn remove(&mut self, name: &str) -> Result<(), CompilerErrors> {
        self.stack.get(name).ok_or(CompilerErrors::MissingVar(String::from(name)))?;
        self.stack.remove(name);
        self.stack_alloc.borrow_mut().free();
        Ok(())
    }

    pub fn get_stack_address(&self, name: &str) -> Result<&usize, CompilerErrors> {
        self.stack.get(name).ok_or(CompilerErrors::MissingVar(String::from(name)))
    }
}

pub struct SymbolTable {
    stack_alloc : std::rc::Rc<RefCell<StackAllocCounter>>,
    pub visible_stack: VarStack,// Symbols visible to the program being compiled (visible)
    pub hidden_stack: VarStack, // Symbols not visible to the program being compiled (Hidden)
    pub functions: HashMap<String, FunctionType>,
    pub anim_stack: Stack<AnimPair>
}

impl SymbolTable { 
    pub fn new() -> SymbolTable{
        let normal_fn = std::iter::repeat(FunctionType::NORMAL);
        let stack_alloc = std::rc::Rc::new(RefCell::new(StackAllocCounter::new()));

        return SymbolTable{
            stack_alloc: stack_alloc.clone(),
            visible_stack : VarStack::new(stack_alloc.clone()),
            hidden_stack : VarStack::new(stack_alloc.clone()),
            functions : HashMap::from_iter(
                GLOBAL_EXTERNAL_FUNCTIONS
                .iter()
                .cloned()
                .map(|f|{String::from(f)})
                .zip(normal_fn)),
            anim_stack : Stack::new()
        }
    }

    pub fn get_stack_allocs(&self) -> usize {
        self.stack_alloc.borrow().get()
    }

}