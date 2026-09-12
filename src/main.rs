mod codegen;
pub mod optimizations;
mod tokenizer;

use clap::{ArgAction, Parser};
use codegen::*;
use optimizations::*;
use rand::RngExt;
use std::process::Command;
use tokenizer::*;

const BFSTDLIB_CONTENT: &str = include_str!("../resources/bfstdlib.h");

#[derive(Parser)]
struct Cmd {
    input: String,

    #[arg(short, long, default_value = "a.out")]
    output: String,

    /// Do not remove build area, show its path
    #[arg(short, long)]
    not_remove_build_area: bool,

    /// Copy generated code to current directory
    #[arg(short, long)]
    copy_compiled: bool,

    /// Tokenize only
    #[arg(short, action=ArgAction::Count)]
    tokenize: u8,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cmd::parse();

    let code: String;

    {
        let tokens =
            tokenize(std::fs::read_to_string(args.input).expect("Failed to read source file"))
                .expect("Tokenize failed");

        if args.tokenize == 1 {
            println!("{:?}", tokens);
            return Ok(());
        }

        let opt_tokens = tokens.optimize();

        if args.tokenize >= 2 {
            println!("{:?}", opt_tokens);
            return Ok(());
        }

        code = opt_tokens.codegen();
    }

    let (path, session_id) = create_build_area().expect("Failed to create build area");

    std::fs::write(format!("{path}/bfstdlib.h"), BFSTDLIB_CONTENT)
        .expect("Failed to write stdlib content");

    std::fs::write(format!("{path}/out.c"), code).expect("Failed to write source code");

    let cmd = Command::new("cc")
        .arg(format!("{path}/out.c"))
        .arg(format!("-o{}", args.output))
        .status()
        .expect("Failed to execute C compiler");

    if !cmd.success() {
        panic!("Compilation failed\nCode: {}", cmd.code().unwrap_or(0));
    }

    if args.copy_compiled {
        std::fs::copy(format!("{path}/out.c"), "./out.c").expect("Failed to copy generated code");
    }

    if !args.not_remove_build_area {
        remove_build_area(session_id).expect("Failed to remove build area");
    } else {
        println!("Session_path: {path}");
    }

    Ok(())
}

fn create_build_area() -> Result<(String, String), Box<dyn std::error::Error>> {
    let session_id = rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(12)
        .map(char::from)
        .collect::<String>();

    let path = format!("/tmp/bf_build_area/session/{}", session_id);

    std::fs::create_dir_all(path.clone())?;

    Ok((path, session_id))
}

fn remove_build_area(session_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("/tmp/bf_build_area/session/{}", session_id);

    std::fs::remove_dir_all(path)?;

    Ok(())
}
