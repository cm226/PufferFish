use std::fmt;

pub struct Instruction { 
  pub instruction : String,
  pub args : Vec<String>
}

impl Instruction { 
  fn to_string(&self) -> String{ // Impl display fmt?
    return format!("{} {}", self.instruction, self.args.join(" "))
  }
}

struct Data { 

}

pub struct Generator { 
  section_text : Vec<Instruction>,
  section_data : Vec<Data>
}

impl Generator { 

  pub fn new() -> Generator { 
    return Generator{
      section_text : Vec::new(),
      section_data : Vec::new()
    }
  }

  fn generate_text_section(&self, output : &mut String) { 
    let _ = fmt::write(output, format_args!("section .text\n"));
    let _ = fmt::write(output, format_args!("    global _start\n"));
    let _ = fmt::write(output, format_args!("_start:\n"));

    for instruction in &self.section_text {
      let _ = fmt::write(output, format_args!("{}\n", instruction.to_string()));
    }
  }

  pub fn generate(&self) {
    let mut output = String::new();
    self.generate_text_section(&mut output);
    println!("{}",output);
  }

  pub fn add_inst(&mut self, inst: Instruction) {
    self.section_text.push(inst)
  }

}