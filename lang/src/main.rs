mod cli;
mod lexer;
mod parser;
mod pratt_parser;
mod semantic;
mod token;
mod mtree;

use clap::Parser;

fn main() {
    // parse CLI
    let args: cli::Cli = cli::Cli::parse();

    // get semantic tree from the command
    let tree = cli::handle(args);

    // symbol table
    let mut sym_table = semantic::SymbolTable::new();

    // run semantic analysis and report how many errors we found
    match semantic::analyze(&tree, &mut sym_table) {
        Ok(_) => println!("\n✓ Semantic analysis completed with 0 error(s)."),
        Err(errors) => {
            println!("\n✓ Semantic analysis completed with {} error(s):", errors.len());
            for (i, error) in errors.iter().enumerate() {
                println!("  {}. {}", i + 1, error);
            }
        }
    }
}






