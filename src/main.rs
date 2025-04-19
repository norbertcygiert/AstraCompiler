#![allow(dead_code, unused_variables)]
use crate::compilation_unit::CompilationUnit;
mod syntax_tree;
mod diagnostics;
mod code;
mod compilation_unit;

fn main() {
    let input = code::source_code::read_sourcefile();
    let compilation_unit = CompilationUnit::compile(&input);
    compilation_unit.run_if_valid();
}