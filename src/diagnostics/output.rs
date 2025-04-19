use std::cmp;
use termion::color;
use crate::diagnostics::Diagnostic;
use crate::code::source_code::SourceCode;

const PREFIX_LENGTH: usize = 8;

pub struct DiagnosticsPrinter<'a> {
    code: &'a SourceCode,
    diagnostics: &'a [Diagnostic],
}


impl <'a> DiagnosticsPrinter<'a> {
    pub fn new(code: &'a SourceCode, diagnostics: &'a [Diagnostic]) -> Self {
        Self { code, diagnostics }
    }

    pub fn stringify_diagnostic(&self, diagnostic: &Diagnostic) -> String {
        let line_index = self.code.line_index(diagnostic.span.start);
        let line = self.code.get_line(line_index);
        let line_start = self.code.line_start(line_index);

        let column = diagnostic.span.start - line_start;

        let (prefix, span, suffix) = self.get_text_spans(diagnostic, &line, column);

        let indent = cmp::min(PREFIX_LENGTH, column);
        let arrow_pointers = Self::format_arrow(diagnostic, indent);
        let error_msg = Self::format_error_message(diagnostic, indent, column, line_index);
        return format!("{}{}{}{}{}\n{}\n{}{}{}", prefix, color::Fg(color::Red), span, color::Fg(color::Reset), suffix, arrow_pointers, color::Fg(color::LightRed), error_msg, color::Fg(color::Reset),);
    }

    fn format_error_message(diagnostic: &Diagnostic, indent: usize, column: usize, line: usize) -> String {
        return format!("{:indent$}{} at ({},{})", "", diagnostic.message, line, column, indent = indent);
    }

    fn format_arrow(diagnostic: &Diagnostic, indent: usize) -> String {
        let arrow_pointers = format!("{:indent$}{}", "", std::iter::repeat('^')
        .take( diagnostic.span.length())
        .collect::<String>(), indent = indent);
    
        return arrow_pointers;
    }

    fn get_text_spans(&'a self, diagnostic: &Diagnostic, line: &'a str, column: usize) -> (&'a str, &'a str, &'a str) {
        let prefix_start = cmp::max(0, column as isize - PREFIX_LENGTH as isize) as usize;
        let prefix_end = column;

        let suffix_start = cmp::min(column + diagnostic.span.length(), line.len());
        let suffix_end = cmp::min(suffix_start + PREFIX_LENGTH, line.len());

        let prefix = &line[prefix_start..prefix_end];
        let span = &line[prefix_end..suffix_start];
        let suffix = &line[suffix_start..suffix_end];

        return (prefix, span, suffix);
    }

    pub fn print(&self) {
        for diagnostic in self.diagnostics {
            println!("{}", self.stringify_diagnostic(diagnostic));
        }
    }
}