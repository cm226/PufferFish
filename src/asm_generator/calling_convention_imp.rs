use core::slice;

use crate::ast_parser::{ast_util::{push_reg_to_stack, with_alligned_stack}, symbol_table};

use super::{asm_helpers::INSTRUCTION, code_generator::{Generator, Instruction}};

#[derive(Clone)]
#[allow(dead_code)]
pub enum Args<'a> { 
    Int(u32),
    StrPtr(&'a str),
    Float(f64),
    FloatReg(& 'a str),
    FloatStack(String)
}

const INT_REGISTERS: &'static [&str] = &["RDI", "RSI", "RDX", "RCX", "R82", "R9"];
const FLOAT_REGISTERS: &'static [&str] = &["XMM0", "XMM1", "XMM2", "XMM3", "XMM4", "XMM5", "XMM6", "XMM7"];

pub fn push_values_from_arg_reg_into_stack<'a>(
    args : core::slice::Iter<& 'a str>,
    gen : &mut Generator,
    scope: &mut symbol_table::SymbolTable
)->Result<(), String> 
{ 
    let mut available_int_registers = Vec::from(INT_REGISTERS);
    available_int_registers.reverse();
    let mut available_float_registers=  Vec::from(FLOAT_REGISTERS);
    available_float_registers.reverse();

    for arg in args { 
        let reg = available_float_registers.pop().ok_or("Ran out of float regisers!!")?;
        gen.add_inst(Instruction::from(INSTRUCTION::MOVQ, ["rdx", reg]));
        push_reg_to_stack(arg, scope, gen, "rdx");
    } 
    Ok(())
}

pub fn call_with<'a>(fn_name : &str, args : core::slice::Iter<Args<'a>>, gen : &mut Generator, scope: &mut symbol_table::SymbolTable) -> Result<(), String> 
{ 
    let mut available_int_registers = Vec::from(INT_REGISTERS);
    available_int_registers.reverse();
    let mut available_float_registers=  Vec::from(FLOAT_REGISTERS);
    available_float_registers.reverse();
    let float_regs = available_float_registers.clone();

    for arg in args {
        match arg {
            Args::Int(i) => {  
                
                let reg = available_int_registers.pop().ok_or("ran out of registers, need impl")?;
                gen.add_inst(
                    Instruction::from(INSTRUCTION::MOV, 
                    [reg,format!("{}",i).as_str()])
                );
            }, 
            Args::StrPtr(str_ptr) => {

                let reg = available_int_registers.pop().ok_or("ran out of registers, need impl")?;
                gen.add_inst(
                    Instruction::from(INSTRUCTION::MOV, 
                    [reg,str_ptr]
                ));
            },
            Args::Float(flt) => {
                
                let reg = available_float_registers.pop().ok_or("ran out of float registers, need impl")?;
                gen.add_inst(
                    Instruction::from(INSTRUCTION::MOV, 
                    [reg,format!("__float64__({}",flt).as_str()]
                ));
            },
            Args::FloatReg(flt_reg) => {

                let reg = available_float_registers.pop().ok_or("ran out of float registers, need impl")?;
                gen.add_inst(
                    Instruction::from(INSTRUCTION::MOVAPD, 
                    [reg,flt_reg]
                ));
            },
            Args::FloatStack(name) => {

                let offset = scope.stack.get(name).ok_or("Invalid var name")?;
                let reg = available_float_registers.pop().ok_or("ran out of float registers, need impl")?;
                gen.add_inst(Instruction::from(INSTRUCTION::MOVQ,[reg,&format!("[rbp-{}]",offset)]));
            }
        }
    }
    gen.add_inst(
        Instruction::from(INSTRUCTION::MOV, 
            ["RAX",
            format!("{}", float_regs.len() - available_float_registers.len()).as_str()
    ]));


    with_alligned_stack(&scope, gen, &|gen : &mut Generator| {
        gen.add_inst(Instruction::from(INSTRUCTION::CALL, [fn_name]));
    });
    return Ok(());

}