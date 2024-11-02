use crate::ast_parser::{ast_util::with_alligned_stack, symbol_table};

use super::{asm_helpers::INSTRUCTION, code_generator::{Generator, Instruction}};

#[allow(dead_code)]
pub enum Args<'a> { 
    Int(u32),
    StrPtr(&'a str),
    Float(f64),
    FloatReg(& 'a str)
}

pub fn call_with<'a, A>(fn_name : &str, args : A, gen : &mut Generator, scope: &mut symbol_table::SymbolTable) -> Result<(), String> 
    where A : IntoIterator<Item = Args<'a>>
{ 
    let mut available_int_registers = vec!("RDI", "RSI", "RDX", "RCX", "R82", "R9");
    available_int_registers.reverse();
    let mut available_float_registers=  vec!("XMM0", "XMM1", "XMM2", "XMM3", "XMM4", "XMM5", "XMM6", "XMM7");
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