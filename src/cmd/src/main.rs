use clap::{AppSettings, Parser};
use lexer::*;
use parser::*;

#[derive(Parser)]
#[clap(
    name = "The Ei Programming Language",
    author = "Matthew Healy",
    version = "0.0.1",
    about = "The larval form of a dependently-typed scripting language.",
    long_about = "Ei is currently just a lexer & parser, \
                  but maybe one day it will be dependently-typed \
                  interpreted scripting language."
)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Cli {
    #[clap(subcommand)]
    invocation: Invocation,
}

#[derive(clap::Subcommand)]
enum Invocation {
    #[clap(
        about = "Dump a pretty-printed debug description of the abstract syntax tree of the provided .ei file."
    )]
    DumpAst { path: std::path::PathBuf },
    #[clap(about = "Typecheck & run the provided .ei file.")]
    Run { path: std::path::PathBuf },
}

fn main() {
    let cli = Cli::parse();

    match cli.invocation {
        Invocation::DumpAst { path } => {
            // TODO: don't read the whole file in at once.
            let contents = std::fs::read_to_string(path).expect("Could not read provided file.");
            let tokens = token_stream(contents.as_str());
            let program = parse(tokens);
            println!("{}", program.pretty_printed());
        }
        Invocation::Run { path } => {
            let contents = std::fs::read_to_string(path).expect("Could not read provided file.");
            let tokens = token_stream(contents.as_str());
            let program = parse(tokens);
            println!("{}", program.pretty_printed());
        }
    }
}

#[test]
fn verify_clap_config() {
    use clap::IntoApp;
    Cli::into_app().debug_assert()
}
