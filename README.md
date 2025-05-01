# Astra - Compiler for a Swift-like language.<br>

## Motivation
I was thinking about a project like this for a long time and the language I will be making this in is Rust - in my eyes a very promising language to get into. <br>
This project is an opportunity for me to learn Rust and Git usage at the same time, as well as prepare for internships and deepen the knowledge about compilers.

## Usage 
1. Clone the repository <br>
```bash
git clone https://github.com/norbertcygiert/AstraCompiler.git
cd AstraCompiler
```
2. Build the project <br>
```bash
cargo build
```
3. Run the AstraScript (.astra) file <br>
```bash
cargo run -- <filename>
```
<b>For help use:</b> 
```bash
cargo run -- -h 
```

## Roadmap 
- [x] Lexer<br>
- [x] Parser<br>
- [x] Error reporting<br>
- [x] Variables<br>
- [ ] Unary expressions<br>
- [ ] Conditional statements<br> 
- [ ] "While" loops<br>
- [ ] Introduce AstraScript<br>
- [ ] Type checking<br>
- [ ] Functions<br>
- [ ] Full AstraScript language design<br>
- [ ] Transpiler (?) <br>

## References
### Julian Hartl's ["Bulding a compiler in Rust"](https://www.youtube.com/playlist?list=PLI1h1vRqlHLNZAa2BEM9uZ2GEvUNYDasO) playlist and "natrixcc" compiler collection
https://github.com/julian-hartl/natrixcc
### Official Rust-Lang Documentation
https://doc.rust-lang.org/stable/std/
