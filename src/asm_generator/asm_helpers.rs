
use super::code_generator::{Generator, Instruction};

use strum_macros::Display;

#[derive(Display)]
pub enum INSTRUCTION { 
  MOV,
  INT,
  ADD,
  MUL
}

pub fn gen_std_out(register : &str,size: u32, gen : &mut Generator) { 

    gen.add_inst(Instruction{
        instruction:INSTRUCTION::MOV,
        args:vec!["[print]".to_string(),register.to_string()]
    });

    gen.add_inst(Instruction{
      instruction:INSTRUCTION::ADD,
      args:vec!["byte [print]".to_string(),"'0'".to_string()]
  });

    gen.add_inst(Instruction{
        instruction:INSTRUCTION::MOV,
        args:vec!["edx".to_string(), size.to_string()]
    });

    if register != "ecx" {
      gen.add_inst(Instruction{
        instruction:INSTRUCTION::MOV,
        args:vec!["ecx".to_string(), "print".to_string()]
      });
    }

    gen.add_inst(Instruction{
        instruction:INSTRUCTION::MOV,
        args:vec!["ebx".to_string(), "1".to_string()]
    });

    gen.add_inst(Instruction{
        instruction:INSTRUCTION::MOV,
        args:vec!["eax".to_string(), "4".to_string()]
    });

    gen.add_inst(Instruction{
      instruction:INSTRUCTION::INT,
      args:vec!["0x80".to_string()]
  });
}