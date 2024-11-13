use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerErrors {
    #[error("The function `{0}` is not defined")]
    MissingFunction(String),
    #[error("The variable `{0}` is not defined")]
    MissingVar(String),
    #[error("Compiler impl limitation - ran out of registers")]
    OutOfRegisters()
}