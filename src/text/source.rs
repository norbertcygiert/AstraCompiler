use std::ops::Index;

pub struct SourceCode{
    text: String,
}

impl SourceCode{
    pub fn new(text: String) -> Self { Self { text } }
    pub fn get_text(&self) -> &String { &self.text }
    
    pub fn get_line(&self, line: usize) -> &str {
        self.text.lines().nth(line).unwrap_or("")
    }

    pub fn line_index(&self, pos: usize) -> usize {
        self.text[..pos].lines().count()
    }
    
    pub fn line_start(&self, index: usize) -> usize {
        self.text.lines().take(index).map(|line| line.len()).sum()
    }
}