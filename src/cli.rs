use ram_machine::error::ParserErrorChain;
use ram_machine::parser::{CodeParseError, RamCode};
use std::ops::Add;
use std::path::PathBuf;
use std::{fs, io};
use thiserror::Error;

use ram_machine::interpreter::{RamMachine, RamMachineError};

use clap::{Command, CommandFactory, Parser as ClapParser, Subcommand, ValueHint};
use clap_complete::{generate, Generator, Shell};

#[derive(ClapParser, Debug)]
#[command(name = "ram", author, version, about, long_about = None)]
struct Cli {
    /// Specifies the path to the input file from which data will be read (input passed from the command line takes precedence)
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    input_file: Option<PathBuf>,

    /// Specifies the path to the output file where the results will be written
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    output_file: Option<PathBuf>,

    /// Don't pass code output to STDOUT
    #[arg(short, long)]
    quiet: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run ram machine code from file
    Run { file: PathBuf, input: Vec<i64> },

    /// Validates ram code syntax of a given file
    Check { file: PathBuf },

    /// Generate a shell completion file
    Init { shell: Shell },
    // Repl,
    // Debug,
}

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Failed to convert `{0}` to an integer")]
    ConvertInputError(String),

    #[error(transparent)]
    CodeParseError(#[from] CodeParseError),

    #[error(transparent)]
    RamMachineError(#[from] RamMachineError),

    #[error("Could not read input from file: '{0}'")]
    ReadInputError(io::Error),

    #[error("Could not read ram machine code from file: '{0}'")]
    ReadCodeError(io::Error),

    #[error("Could not write output to file: '{0}'")]
    WriteOutputFileError(io::Error),

    #[error("{0}")]
    CheckFileError(ParserErrorChain),
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

pub fn app() -> Result<(), RuntimeError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file, input } => {
            let mut input = input;

            if let Some(input_file) = cli.input_file {
                let file =
                    fs::read_to_string(input_file).map_err(|e| RuntimeError::ReadInputError(e))?;
                for s in file.split_whitespace() {
                    input.push(
                        s.parse::<i64>()
                            .map_err(|_| RuntimeError::ConvertInputError(s.to_string()))?,
                    );
                }
            }

            let unparsed_file =
                fs::read_to_string(file).map_err(|e| RuntimeError::ReadCodeError(e))?;
            let interpreter = RamMachine::from_str(&unparsed_file, input)?;
            let output = interpreter.run()?;

            if !cli.quiet {
                println!("{:?}", output);
            }

            if let Some(output_file) = cli.output_file {
                fs::write(
                    output_file,
                    output
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" "),
                )
                .map_err(|e| RuntimeError::WriteOutputFileError(e))?;
            }
        }
        Commands::Check { file } => {
            let unparsed_file =
                fs::read_to_string(file).map_err(|e| RuntimeError::ReadCodeError(e))?;
            let mut errors = ParserErrorChain::new();

            let mut code = RamCode::new();
            let lines = unparsed_file.lines().into_iter().enumerate();
            for (index, line) in lines {
                if line.is_empty() {
                    continue;
                };
                code.push_line(line).unwrap_or_else(|err| {
                    errors.add((
                        index
                            .add(1)
                            .try_into()
                            .expect("Could not convert the line number to u32"),
                        err,
                    ))
                });
            }

            if !errors.is_empty() {
                return Err(RuntimeError::CheckFileError(errors));
            }
        }
        Commands::Init { shell } => {
            let mut cmd = Cli::command();
            eprintln!("Generating completion file for {shell:?}...");
            print_completions(shell, &mut cmd)
        }
    };
    Ok(())
}
