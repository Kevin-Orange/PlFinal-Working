# Language Compiler Project

A minimalistic imperative programming language with lexer, parser, and semantic analyzer.

## Building

```bash
cd /path/to/project
cargo build --release
```

## Running

From the repository root, use the convenient wrapper:

### Print (default test file)
```bash
cargo run print
```

### Parse with verbose output
```bash
cargo run parse lex.txt
```

### Tokenize
```bash
cargo run tokenize lex.txt
```

### Custom files
To run on a custom file, specify the path:
```bash
cargo run parse path/to/myfile.tpl
```

The wrapper automatically resolves paths under `lang/src/` or `lang/`, so you can run:
```bash
cargo run parse myfile.tpl
```
and it will find `lang/src/myfile.tpl` if it exists.

### Alternative (direct invocation)
If you prefer to invoke the compiler directly:
```bash
cd lang
cargo run -- parse src/lex.txt
```

## Features

- **Lexical Analysis**: Hand-coded FSM lexer in `lang/src/lexer.rs`
- **Parsing**: Recursive descent parser (`parser.rs`) with Pratt expression parsing (`pratt_parser.rs`)
- **Semantic Analysis**: Type checking, variable declaration verification, function arity checking

## Test File

The included test file (`lang/src/lex.txt`) demonstrates all language features including error cases for semantic analysis testing.

## Project Structure

```
lang/
  src/
    main.rs          - Entry point, orchestrates CLI and analysis
    cli.rs           - CLI command handler
    lexer.rs         - FSM-based tokenizer
    token.rs         - Token definitions
    parser.rs        - Recursive descent parser
    pratt_parser.rs  - Pratt precedence climbing for expressions
    semantic.rs      - Semantic analysis (type checking, etc.)
    mtree.rs         - Parse tree representation
    lex.txt          - Test input file
  Cargo.toml         - Rust dependencies
src/
  bin/
    print.rs         - Wrapper binary for convenient CLI
Cargo.toml           - Root workspace config
```

## Language Overview

The language supports:
- Functions with parameters and return types
- Integer (`i32`) and boolean (`bool`) types
- Arithmetic: `+`, `-`, `*`, `/`
- Relational: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical: `&&`, `||`, `!`
- Control flow: `if`-`else`, `while`, `return`
- Variable declarations: `let x: i32 = 5;`
- Assignments: `x = 10;`
- Function calls: `factorial(n)`
- Print statement: `print result;`

## Grammar Specification (EBNF)

The following EBNF matches the current implementation in `lang/src/{lexer.rs, parser.rs, pratt_parser.rs}`.

Notation: { } = repetition (0..n), [ ] = optional (0..1), | = alternative, "..." = terminal.

Program
<program> ::= { <func_decl> }

Functions
<func_decl> ::= "func" <id> "(" [ <params> ] ")" [ "->" <type> ] <block>
<params> ::= <param> { "," <param> }
<param> ::= <id> ":" <type>

Types
<type> ::= "i32" | "f32" | "char" | "bool"

Statements and Blocks
<block> ::= "[" { <stmt> } "]"
<stmt> ::= <let_stmt> | <if_stmt> | <while_stmt> | <return_stmt> | <print_stmt> | <expr_stmt>

<let_stmt> ::= "let" <id> [ ":" <type> ] [ "=" <expr> ] ";"
<if_stmt> ::= "if" <expr> <block> [ "else" <block> ]
<while_stmt> ::= "while" <expr> <block>
<return_stmt> ::= "return" <expr> ";"
<print_stmt> ::= "print" <expr> ";"
<expr_stmt> ::= <expr> ";"

Expressions (precedence handled by Pratt parser)
<expr> ::= <assign>
<assign> ::= <or_expr> | <id> "=" <assign>
<or_expr> ::= <and_expr> { "||" <and_expr> }
<and_expr> ::= <eq_expr> { "&&" <eq_expr> }
<eq_expr> ::= <rel_expr> { ( "==" | "!=" ) <rel_expr> }
<rel_expr> ::= <add_expr> { ( "<" | ">" | "<=" | ">=" ) <add_expr> }
<add_expr> ::= <mul_expr> { ( "+" | "-" ) <mul_expr> }
<mul_expr> ::= <unary> { ( "*" | "/" ) <unary> }
<unary> ::= <primary> | ( "!" | "-" ) <unary>
<primary> ::= <id> | <literal> | <call> | "(" <expr> ")"
<call> ::= <id> "(" [ <args> ] ")"
<args> ::= <expr> { "," <expr> }

Lexical (informal EBNF)
<id> ::= ( letter | "_" ) { letter | digit | "_" | "-" }
<literal> ::= <int> | <float> | <char> | <bool> | <string>
<int> ::= digit { digit }
<float> ::= digit { digit } "." digit { digit }
<char> ::= "'" character "'"
<bool> ::= "true" | "false"
<string> ::= '"' { character } '"'
letter ::= "a".."z" | "A".."Z"
digit ::= "0".."9"

Comments
"//" { character } (line comment, discarded by lexer)

Precedence (high → low): primary, unary (! -), mult (* /), add (+ -), relational (< > <= >=), equality (== !=), logical (&& ||), assignment (=)

-- Note: conditionals do not require parentheses around the condition in this implementation.
<let_stmt> ::= "let" <id> [":" <type>] ["=" <expr>] ";"
<if_stmt> ::= "if" <expr> <block> ["else" <block>]
<while_stmt> ::= "while" <expr> <block>
<return_stmt> ::= "return" <expr> ";"
<print_stmt> ::= "print" <expr> ";"
<expr_stmt> ::= <expr> ";"

Expressions (operator precedence handled by Pratt parser)
<expr> ::= <assign>
<assign> ::= <or_expr> | <id> "=" <assign>
<or_expr> ::= <and_expr> ["||" <and_expr>...]
<and_expr> ::= <eq_expr> ["&&" <eq_expr>...]
<eq_expr> ::= <rel_expr> [("==" | "!=") <rel_expr>...]
<rel_expr> ::= <add_expr> [("<" | ">" | "<=" | ">=") <add_expr>...]
<add_expr> ::= <mul_expr> [("+" | "-") <mul_expr>...]
<mul_expr> ::= <unary> [("*" | "/") <unary>...]
<unary> ::= <primary> | "!" <unary> | "-" <unary>
<primary> ::= <id> | <literal> | <call> | "(" <expr> ")"
<call> ::= <id> "(" [<args>] ")"
<args> ::= <expr> ["," <expr>...]

Lexical
<id> ::= (letter | "_") [letter | digit | "_" | "-"...]
<literal> ::= <int> | <float> | <char> | <bool> | <string>
<int> ::= digit [digit...]
<float> ::= digit [digit...] "." digit [digit...]
<char> ::= "'" char "'"
<bool> ::= "true" | "false"
<string> ::= '"' [char...] '"'
letter ::= "a".."z" | "A".."Z"
digit ::= "0".."9"

Comments
// ... (line comment, discarded during lexing)

Precedence (high to low): primary, unary (! -), mult (* /), add (+ -), relational (< > <= >=), equality (== !=), logical (&& ||), assignment (=)


## Example

```tpl
func factorial(n: i32) -> i32 [
    if n < 2 [
        return 1;
    ] else [
        return n * factorial(n - 1);
    ]
]

func main() -> i32 [
    let result: i32 = factorial(5);
    print result;
    return result;
]
```

## Semantic Analysis Output

The compiler reports semantic errors with details:

```
✓ Semantic analysis completed with 3 error(s):
  1. Variable 'undefined_var' not declared
  2. Type mismatch for 'x': expected Int, found Bool
  3. Function 'unknown_func' expects 1 arg but 2 provided
```

For details on coverage of assignment requirements, see `REQUIREMENTS_COVERAGE.md`.
