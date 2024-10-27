use std::{borrow::Borrow, fmt};

use super::asm_helpers::INSTRUCTION;

fn args_from_borrowable<S,T>(args : S) -> Vec<String>
where
  S: IntoIterator<Item = T>,
  T : Borrow<str>
{
  let mut args_v = Vec::new();
  for arg in args{
    args_v.push(String::from(arg.borrow()));
  }
  args_v
}

pub struct Instruction { 
  pub instruction : INSTRUCTION,
  pub args : Vec<String>
}

impl fmt::Display for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{} {}", self.instruction, self.args.join(","))
  }
}

impl Instruction { 
  pub fn from<S, T>(instruction : INSTRUCTION, args: S) -> Self
  where
    S: IntoIterator<Item = T>,
    T : Borrow<str>
    {
      Instruction{
        instruction : instruction,
        args: args_from_borrowable(args)
      }
    }
}

pub struct Label { 
  pub name : String,
}

impl fmt::Display for Label {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}:", self.name)
  }
}

impl Label { 
  pub fn from<T>(name: T) -> Self
  where
    T : Borrow<str>
    {
      Label{
        name : String::from(name.borrow()),
      }
    }
}

pub struct Data { 
  pub name : String, 
  pub kind : String, 
  pub args : Vec<String>
}

impl Data { 
  #[allow(dead_code)]
  pub fn from<S, T>(name : T, kind : T, args: S) -> Self
  where
    S: IntoIterator<Item = T>,
    T : Borrow<str>
    {
      Data{
        name : String::from(name.borrow()),
        kind : String::from(kind.borrow()),
        args: args_from_borrowable(args)
      }
    }
}

impl fmt::Display for Data {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{} {} {}", self.name, self.kind, self.args.join(","))
  }
}

pub enum TextLines { 
  Instruction(Instruction),
  Label(Label)
}

pub struct Generator { 
  section_text : Vec<TextLines>,
  section_fn : Vec<TextLines>, // Function lines go at the end of the text section so store separately?
  section_data : Vec<Data>,
  section_bss : Vec<Data>
}

pub const GLOBAL_EXTERNAL_FUNCTIONS : &'static [&str] = &["cos", "sin"];

impl Generator { 

  pub fn new() -> Generator { 
    return Generator{
      section_text : Vec::new(),
      section_fn : Vec::new(),
      section_data : Vec::new(),
      section_bss : Vec::new()
    }
  }

  fn generate_text_section(&self, output : &mut String) { 
    let _ = fmt::write(output, format_args!("section .text\n"));
    let _ = fmt::write(output, format_args!("    global main\n"));
    let _ = fmt::write(output, format_args!("main:\n"));

    for instruction in &self.section_text {
      let _ = match instruction{
        TextLines::Instruction(instruct) => fmt::write(output, format_args!("{}\n", instruct)),
        TextLines::Label(label) => fmt::write(output, format_args!("{}\n", label))
      };
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

  fn generate_externs_section(&self, output: &mut String){
    let _ = fmt::write(output, format_args!("extern draw_shape\n")); // from graphics lib
    let _ = fmt::write(output, format_args!("extern create_window\n")); // from graphics lib 
    let _ = fmt::write(output, format_args!("extern destroy_window\n")); // from graphics lib
    let _ = fmt::write(output, format_args!("extern blit\n")); // from graphics lib
    let _ = fmt::write(output, format_args!("extern clear\n")); // from graphics lib
    let _ = fmt::write(output, format_args!("extern printf\n")); // from c
    let _ = fmt::write(output, format_args!("extern fflush\n")); // from c
    for global_extern in GLOBAL_EXTERNAL_FUNCTIONS { 
      let _ = fmt::write(output, format_args!("extern {}\n", global_extern)); // from c
    }
  }

  pub fn generate(&mut self) -> String {
    let mut output = String::new();

    // Add the printf format
    self.add_data(Data::from("print_fmt_str", "db", ["`%f\\n\\0`","0"]));

    // Add the exit code 
    self.add_inst(Instruction{
      instruction : INSTRUCTION::MOV,
      args : vec!["eax".to_string(), "1".to_string()]
    });
    
    self.add_inst(Instruction{
      instruction : INSTRUCTION::MOV,
      args : vec!["ebx".to_string(), "0".to_string()]
    });

    self.add_inst(Instruction{
        instruction: INSTRUCTION::INT,
        args:vec!["0x80".to_string()]
    });

    self.section_text.append(&mut self.section_fn);

    self.generate_externs_section(&mut output);
    self.generate_text_section(&mut output);
    let _ = fmt::write(&mut output, format_args!("\n"));
    self.generate_data_section(&mut output);
    let _ = fmt::write(&mut output, format_args!("\n"));
    self.generate_bss_section(&mut output);

    output
  }

  pub fn add_inst(&mut self, inst: Instruction) {
    self.section_text.push(TextLines::Instruction(inst))
  }

  pub fn add_label(&mut self, label: Label) {
    self.section_text.push(TextLines::Label(label))
  }

  #[allow(dead_code)]
  pub fn add_bss(&mut self, data: Data) {
    self.section_bss.push(data)
  }

  pub fn add_data(&mut self, data: Data) {
    self.section_data.push(data);
  }

  pub fn append(&mut self, gen: &mut Generator) {
    self.section_fn.append(&mut gen.section_text);
    self.section_bss.append(&mut gen.section_bss); // TODO implement proper stack not this BS
  }

}