pub struct ArcticPalette {
  palettes: Vec<Palette>,
  fallback: Option<Palette>
}

struct Palette {
  name: String,
  handler: Box<dyn Fn(Command)>
}

pub struct Command {
  pub arguments: Vec<String>,
  pub flags: Vec<String>,
  pub command: String,
}

impl Command {
  pub fn new(raw: String) -> Self {
    let mut segments: Vec<String> = raw.split_whitespace().map(String::from).collect();
    let command = segments.remove(0);
    let mut flags: Vec<String> = Vec::new();
    let mut arguments: Vec<String> = Vec::new();
    for segment in &segments {
      if segment.starts_with("-") {
        flags.push(segment[1..].to_string());
      } else {
        arguments.push(segment.to_string());
      }
    }
    Command { arguments, command, flags }
  }
  pub fn get_cmd(&self) -> String {
    self.command.clone()
  }
  pub fn get_arg(&self, index: usize) -> Option<String> {
    self.arguments.get(index).cloned()
  }
  pub fn pop_arg(&mut self, index: usize) -> Option<String> {
    if index < self.arguments.len() {
      Some(self.arguments.remove(index))
    } else {
      None
    }
  }
  pub fn join_args(&self) -> String {
    self.arguments.join(" ")
  }
  pub fn get_flags(&self) -> Vec<String> {
    self.flags.clone()
  }
  pub fn has_flag(&self, flag: String) -> bool {
    self.flags.contains(&flag.to_string())
  }
}

impl ArcticPalette {
  pub const NOT_FOUND: usize = 200;
  pub const FALLBACK: usize = 201;
  pub const SUCCESS: usize = 100;
  pub fn new() -> Self {
    let palettes: Vec<Palette> = Vec::new();
    Self { palettes, fallback: None }
  }
  pub fn assign<F>(&mut self, name: String, handler: F)
  where F: Fn(Command) + 'static {
    let palette = Palette {
      name: name.to_lowercase(),
      handler: Box::new(handler)
    };
    self.palettes.push(palette);
  }
  pub fn assign_fallback<F>(&mut self, handler: F)
  where F: Fn(Command) + 'static {
    let palette = Palette {
      name: String::new(),
      handler: Box::new(handler)
    };
    self.fallback = Some(palette);
  }
  pub fn apply(&self, raw: String) -> usize {
    let cmd = Command::new(raw);
    if let Some(palette) = self.palettes.iter().find(|p| p.name == cmd.command) {
      (palette.handler)(cmd);
      ArcticPalette::SUCCESS
    } else {
      if let Some(fallback) = &self.fallback {
        (fallback.handler)(cmd);
        ArcticPalette::FALLBACK
      } else {
        ArcticPalette::NOT_FOUND
      }
    }
  }
}