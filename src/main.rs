mod token;
mod lexer;
mod ast;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Telugu Language Compiler v1.0");
        println!("Usage: {} <filename.tel>", args[0]);
        println!("   or: {} --help", args[0]);
        println!("\nExample:");
        println!("   {} examples/hello.tel", args[0]);
        process::exit(1);
    }
    
    if args[1] == "--help" || args[1] == "-h" {
        print_help(&args[0]);
        return;
    }
    
    let filename = &args[1];
    
    // Check if file has .tel extension
    if !filename.ends_with(".tel") {
        eprintln!("Error: Telugu files must have .tel extension");
        process::exit(1);
    }
    
    // Read the Telugu source file
    let telugu_code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };
    
    // Check for verbose output flag
    let verbose = args.len() > 2 && (args[2] == "--verbose" || args[2] == "-v");
    
    if verbose {
        println!("Telugu Language Compiler v1.0");
        println!("==============================");
        println!("Compiling: {}\n", filename);
    }
    
    // Lexical analysis
    let mut lexer = Lexer::new(&telugu_code);
    let tokens = lexer.scan_tokens();
    
    if verbose {
        println!("Lexical Analysis:");
        println!("-----------------");
        for token in &tokens {
            println!("{:?}", token);
        }
        println!();
    }
    
    // Parsing
    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(program) => {
            if verbose {
                println!("Parsing: Success");
                println!();
            }
            program
        }
        Err(e) => {
            eprintln!("Parse error in '{}': {}", filename, e);
            process::exit(1);
        }
    };
    
    // Interpretation/Execution
    if verbose {
        println!("Execution Output:");
        println!("----------------");
    }
    
    let mut interpreter = Interpreter::new();
    match interpreter.interpret(&program) {
        Ok(_) => {
            if verbose {
                println!("\n✓ Program executed successfully!");
            }
        }
        Err(e) => {
            eprintln!("Runtime error in '{}': {:?}", filename, e);
            process::exit(1);
        }
    }
}

fn print_help(program_name: &str) {
    println!("Telugu Language Compiler v1.0");
    println!("==============================");
    println!();
    println!("USAGE:");
    println!("    {} <filename.tel> [OPTIONS]", program_name);
    println!();
    println!("OPTIONS:");
    println!("    -v, --verbose    Show detailed compilation steps");
    println!("    -h, --help       Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    {} examples/hello.tel", program_name);
    println!("    {} examples/calculator.tel --verbose", program_name);
    println!();
    println!("Telugu Language Keywords:");
    println!("    sankhya          - Number/Integer variable");
    println!("    cheppu           - Print statement");
    println!("    ayite            - If condition");
    println!("    lekapothe        - Else");
    println!("    varaku           - While loop");
    println!("    pani             - Function");
    println!("    tirigi_pampu     - Return statement");
    println!();
    println!("File Extension: .tel (Telugu Language)");
}
