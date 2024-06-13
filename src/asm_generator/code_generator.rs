use std::fmt;

use super::asm_helpers::INSTRUCTION;

pub struct Instruction { 
  pub instruction : INSTRUCTION,
  pub args : Vec<String>
}

impl fmt::Display for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{} {}", self.instruction, self.args.join(","))
  }
}

pub struct Data { 
  pub name : String, 
  pub kind : String, 
  pub args : Vec<String>
}

impl fmt::Display for Data {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{} {} {}", self.name, self.kind, self.args.join(","))
  }
}

pub struct Generator { 
  section_text : Vec<Instruction>,
  section_data : Vec<Data>,
  section_bss : Vec<Data>
}

impl Generator { 

  pub fn new() -> Generator { 
    return Generator{
      section_text : Vec::new(),
      section_data : Vec::new(),
      section_bss : Vec::new()
    }
  }

  fn generate_text_section(&self, output : &mut String) { 
    let _ = fmt::write(output, format_args!("section .text\n"));
    let _ = fmt::write(output, format_args!("    global _start\n"));
    let _ = fmt::write(output, format_args!("_start:\n"));

    for instruction in &self.section_text {
      let _ = fmt::write(output, format_args!("{}\n", instruction));
    }
  }

  fn generate_data_section(&self, output : &mut String) { 
    let _ = fmt::write(output, format_args!("segment .data\n"));

    for data in &self.section_data {
      let _ = fmt::write(output, format_args!("{}\n", data));
    }
  }

  fn generate_bss_section(&self, output : &mut String) {
    let _ = fmt::write(output, format_args!("segment .bss\n"));
    for data in &self.section_bss {
      let _ = fmt::write(output, format_args!("{}\n", data));
    }
  }

  pub fn generate(&mut self) -> String {
    let mut output = String::new();

    // Add the exit code 
    self.add_inst(Instruction{
      instruction : INSTRUCTION::MOV,
      args : vec!["eax".to_string(), "1".to_string()]
    });

    self.add_inst(Instruction{
        instruction: INSTRUCTION::INT,
        args:vec!["0x80".to_string()]
    });

    self.generate_text_section(&mut output);
    let _ = fmt::write(&mut output, format_args!("\n"));
    self.generate_data_section(&mut output);
    let _ = fmt::write(&mut output, format_args!("\n"));
    self.generate_bss_section(&mut output);

    output
  }

  pub fn add_inst(&mut self, inst: Instruction) {
    self.section_text.push(inst)
  }

  pub fn add_data(&mut self, data: Data) {
    self.section_data.push(data)
  }

  pub fn add_bss(&mut self, data: Data) {
    self.section_bss.push(data)
  }

}