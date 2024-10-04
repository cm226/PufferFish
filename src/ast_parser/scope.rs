
use pest::Stack;
use std::collections::{HashMap, HashSet};
pub struct Scope {
    pub stack: HashMap<String, usize>,
    pub function: HashSet<String>,
    pub anim_stack: Stack<String>
}

impl Scope { 
    pub fn new() -> Scope{
        return Scope{
            stack : HashMap::new(),
            function : HashSet::new(),
            anim_stack : Stack::new()
        }
    }
}