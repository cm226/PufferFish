use crate::{ast_parser::{ast_util::{push_reg_to_stack, with_aligned_stack}, symbol_table::{self, VarStack}}, errors::compiler_errors::CompilerErrors};

use super::{asm_helpers::INSTRUCTION, code_generator::{Generator, Instruction}};

#[derive(Clone)]
#[allow(dead_code)]
pub enum Args<'a> { 
    Int(u32),
    StrPtr(&'a str),
    Float(f64),
    FloatReg(& 'a str),
    FloatStack(usize),
    IntStack(usize),
    IntReg(& 'a str)
}

const INT_REGISTERS: &'static [&str] = &["RDI", "RSI", "RDX", "RCX", "R82", "R9"];
const FLOAT_REGISTERS: &'static [&str] = &["XMM0", "XMM1", "XMM2", "XMM3", "XMM4", "XMM5", "XMM6", "XMM7"];

pub fn push_values_from_arg_reg_into_stack<'a>(
    args : core::slice::Iter<& 'a str>,
    gen : &mut Generator,
    stack: &mut VarStack
)->Result<(), CompilerErrors> 
{ 
    let mut available_int_registers = Vec::from(INT_REGISTERS);
    available_int_registers.reverse();
    let mut available_float_registers=  Vec::from(FLOAT_REGISTERS);
    available_float_registers.reverse();

    for arg in args { 
        let reg = available_float_registers.pop().ok_or
            (CompilerErrors::OutOfRegisters())?;
        gen.add_inst(Instruction::from(INSTRUCTION::MOVQ, ["rdx", reg]));
        push_reg_to_stack(arg, stack, gen, "rdx");
    } 
    Ok(())
}

pub fn call_with<'a>(fn_name : &str, args : core::slice::Iter<Args<'a>>, gen : &mut Generator, scope: &symbol_table::SymbolTable) -> Result<(), CompilerErrors> 
{ 
    let mut available_int_registers = Vec::from(INT_REGISTERS);
    available_int_registers.reverse();
    let mut available_float_registers=  Vec::from(FLOAT_REGISTERS);
    available_float_registers.reverse();
    let float_regs = available_float_registers.clone();

    for arg in args {
        match arg {
            Args::Int(i) => {  
                
                let reg = available_int_registers.pop().ok_or(CompilerErrors::OutOfRegisters())?;
                gen.add_inst(
                    Instruction::from(INSTRUCTION::MOV, 
                    [reg,format!("{}",i).as_str()])
                );
            }, 
            Args::StrPtr(str_ptr) => {

                let reg = available_int_registers.pop().ok_or(CompilerErrors::OutOfRegisters())?;
                gen.add_inst(
                    Instruction::from(INSTRUCTION::MOV, 
                    [reg,str_ptr]
                ));
            },
            Args::Float(flt) => {
                
                let reg = available_float_registers.pop().ok_or(CompilerErrors::OutOfRegisters())?;
                gen.add_inst(
                    Instruction::from(INSTRUCTION::MOV, 
                    [reg,format!("__float64__({}",flt).as_str()]
                ));
            },
            Args::FloatReg(flt_reg) => {

                let reg = available_float_registers.pop().ok_or(CompilerErrors::OutOfRegisters())?;
                gen.add_inst(
                    Instruction::from(INSTRUCTION::MOVAPD, 
                    [reg,flt_reg]
                ));
            },
            Args::FloatStack(offset) => {
                let reg = available_float_registers.pop().ok_or(CompilerErrors::OutOfRegisters())?;
                gen.add_inst(Instruction::from(INSTRUCTION::MOVQ,[reg,&format!("[rbp-{}]",offset)]));
            }
            Args::IntStack(offset) => {
                let reg = available_int_registers.pop().ok_or(CompilerErrors::OutOfRegisters())?;
                gen.add_inst(Instruction::from(INSTRUCTION::MOV,[reg,&format!("[rbp-{}]",offset)]));
            }
            Args::IntReg(int_reg) => {

                let reg = available_int_registers.pop().ok_or(CompilerErrors::OutOfRegisters())?;
                gen.add_inst(Instruction::from(INSTRUCTION::MOV,[reg,int_reg]));
            }
        }
    }
    gen.add_inst(
        Instruction::from(INSTRUCTION::MOV, 
            ["RAX",
            format!("{}", float_regs.len() - available_float_registers.len()).as_str()
    ]));


    with_aligned_stack(&scope, gen, &|gen : &mut Generator| {
        gen.add_inst(Instruction::from(INSTRUCTION::CALL, [fn_name]));
    });
    return Ok(());

}