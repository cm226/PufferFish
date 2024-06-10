use super::code_generator::{Generator, Instruction};


pub fn gen_std_out(register : &str,size: u32, gen : &mut Generator) { 
    gen.add_inst(Instruction{
        instruction:"mov".to_string(),
        args:vec!["edx".to_string(), size.to_string()]
    });

    if register != "ecx" {
      gen.add_inst(Instruction{
        instruction:"mov".to_string(),
        args:vec!["ecx".to_string(), register.to_string()]
      });
    }

    gen.add_inst(Instruction{
        instruction:"mov".to_string(),
        args:vec!["ebx".to_string(), "1".to_string()]
    });

    gen.add_inst(Instruction{
        instruction:"mov".to_string(),
        args:vec!["eax".to_string(), "4".to_string()]
    });

    gen.add_inst(Instruction{
      instruction:"int".to_string(),
      args:vec!["0x80".to_string()]
  });
}