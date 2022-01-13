use ei_lexer::*;
use ei_parser::*;

enum Invocation {
    Run { path: std::path::PathBuf },
}

impl Invocation {
    fn parse(cmd: &str) -> Option<Invocation> {
        match cmd {
            "run" => {
                let path = std::env::args()
                    .nth(2)
                    .expect("The run command requires a .ei file to function.");
                let path = std::path::PathBuf::from(path);
                Some(Invocation::Run { path })
            }
            _ => None,
        }
    }
}

fn main() {
    let command = std::env::args().nth(1).expect("No first arg!");
    let invocation = Invocation::parse(&command);
    match invocation {
        None => panic!("No such command: {}", command),
        Some(Invocation::Run { path }) => {
            // TODO: don't read the whole file in at once.
            let contents = std::fs::read_to_string(path).expect("Could not read provided file.");
            let tokens = token_stream(contents.as_str());
            let program = parse(tokens);
            println!("{}", program.ast_debug_string());
        }
    }
}
