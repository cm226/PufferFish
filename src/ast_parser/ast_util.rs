use crate::ast_parser::scope::Scope;

pub fn push_var_to_stack(name: &str,scope: &mut Scope ) {

    // I must be missing something here?
    // (scope.len() * 8) i would have thought would be the address of the next thing that will be added to the stack
    // so the +4 shouldn't be needed. But it looks like it is needed. 
    // So when rsb == rsp (theres nothing on the stack) when you add the first thing, you need to use the address rsb + 8? 
    // so thats stored at esb? 
    scope.stack.insert(String::from(name), (scope.stack.len() * 8)+8);
}