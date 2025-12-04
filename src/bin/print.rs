use std::env;
use std::process::{Command, exit};
use std::path::Path;

fn main() {
    // grab args (skip program name)
    let args: Vec<String> = env::args().skip(1).collect();

    // default to printing the included test file when no filepath given
    let mut cmd_args: Vec<String> = Vec::new();
    if args.is_empty() {
        cmd_args.push("print".to_string());
        cmd_args.push("lang/src/lex.txt".to_string());
    } else if args.len() == 1 && args[0] == "print" {
        cmd_args.push("print".to_string());
        cmd_args.push("lang/src/lex.txt".to_string());
    } else {
        // pass args through
        cmd_args.extend(args.iter().cloned());
    }

    // if the file isn't here, try the common locations under `lang/`
    if cmd_args.len() >= 2 {
        let file = cmd_args[1].clone();
        if !Path::new(&file).exists() {
            let candidate = format!("lang/src/{}", file);
            if Path::new(&candidate).exists() {
                cmd_args[1] = candidate;
            } else {
                let candidate2 = format!("lang/{}", file);
                if Path::new(&candidate2).exists() {
                    cmd_args[1] = candidate2;
                }
            }
        }
    }

    // run the `lang` crate via cargo (use its Cargo.toml)
    let status = Command::new("cargo")
        .arg("run")
        .arg("--manifest-path")
        .arg("lang/Cargo.toml")
        .arg("--")
        .args(&cmd_args)
        .status()
        .expect("failed to spawn cargo for lang crate");

    if let Some(code) = status.code() {
        exit(code);
    } else {
        // unexpected termination
        eprintln!("lang process terminated unexpectedly");
        exit(1);
    }
}
