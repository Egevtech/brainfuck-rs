pub mod optimizations;
mod tokenizer;
mod codegen;

use std::process::Command;
use clap::{ Parser};
use optimizations::*;
use tokenizer::*;
use codegen::*;

const BFSTDLIB_CONTENT: &str = include_str!("../resources/bfstdlib.h");

#[derive(Parser)]
struct Cmd {
    input: String,

    #[arg(short, long, default_value = "a.out")]
    output: String,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cmd::parse();

    let code = tokenize(std::fs::read_to_string(args.input).expect("Failed to read source file"))
        .expect("Tokenize failed")
        .optimize()
        .codegen();

    std::fs::write(
        "./bfstdlib.h",
        BFSTDLIB_CONTENT
    ).expect("Failed to write stdlib content");

    std::fs::write(
        "./out.c",
        code
    ).unwrap_or_else(|err| {
        std::fs::remove_file("./bfstdlib.h").expect("Failed to remove stdlib content;; Failed to write source file");
        panic!("Failed to write source file: {err}");
    });

    let cmd = Command::new("cc").arg("out.c").arg(format!("-o{}", args.output)).status().expect("Failed to execute C compiler");

    if !cmd.success() {
        panic!("Compilation failed\nCode: {}", cmd.code().unwrap_or(0));
    }

    std::fs::remove_file("./out.c").expect("Failed to remove source file after compilation");
    std::fs::remove_file("./bfstdlib.h").expect("Failed to remove stdlib file after compilation");

    Ok(())
}
