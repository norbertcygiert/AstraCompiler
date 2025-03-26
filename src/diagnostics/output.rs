use std::cmp;
use termion::color::*;
use crate::diagnostics::Diagnostic;
use crate::text::source::{self, SourceCode};

const PREFIX_LEN: usize = 8; 

pub struct DiagnosticsOutput<'a> {
    text: &'a SourceCode,
    diag: &'a [Diagnostic],
}

impl <'a> DiagnosticsOutput<'a> {
    pub fn new(text: &'a SourceCode, diag: &'a [Diagnostic]) -> Self {
        Self { text, diag }
    }

    pub fn stringify_diagnostic(&self, diag: &Diagnostic) -> String {
        let line_index = self.text.line_index(diag.span.start);
        let line = self.text.get_line(line_index);
        let line_start = self.text.line_start(line_index);

        let col = diag.span.start - line_start;
        let prefix_start = cmp::max(0, col as isize - PREFIX_LEN as isize) as usize;
        let prefix_end = col;
        let suffix_start = cmp::min(col + diag.span.len(), line.len()) + 1;
        let suffix_end = cmp::min(suffix_start + diag.span.len() + PREFIX_LEN, line.len());


        let prefix = &line[prefix_start..prefix_end];
        let suffix = &line[suffix_start..suffix_end];
        let span = &line[prefix_end..suffix_start];


        let indent = cmp::min(PREFIX_LEN, col);
        let pointer = format!("{:indent$}{}", "", std::iter::repeat("^").take(diag.span.len()).collect::<String>(), indent = indent);
        let arrow = format!("{:indent$}|", "", indent = indent);
        let msg = format!("{:indent$}: {}", "" , diag.content, indent = indent);
        format!("{}{}{}{}{}\n{}\n{}\n{}", prefix, termion::color::Fg(Red) ,span, termion::color::Fg(Reset) ,suffix, pointer, arrow, msg)
    }

}