use ast::{Expr, ExprVisitor, Literal, UntypedProgram};
use token::Token;

#[derive(Debug)]
enum Value {
    PrintFn,
    String(String),
    Void,
}

struct Interpreter<'w, W: std::fmt::Write> {
    out: &'w mut W,
}

impl<'w, W: std::fmt::Write> Interpreter<'w, W> {
    fn new(out: &mut W) -> Interpreter<W> {
        Interpreter { out }
    }

    fn interpret(&mut self, p: UntypedProgram) {
        for s in p.stmts {
            match s {
                ast::Stmt::Expr { e } => {
                    e.accept(self);
                    ()
                }
            }
        }
    }
}

impl<'w, W: std::fmt::Write> ExprVisitor<Value> for Interpreter<'w, W> {
    fn visit_identifier(&mut self, name: &Token) -> Value {
        if name.lexeme == "print_ln" {
            Value::PrintFn
        } else {
            panic!("Unknown identifier: {}", name.lexeme)
        }
    }

    fn visit_literal(&mut self, l: &Literal) -> Value {
        match l {
            // TODO(STR_TABLE): avoid this clone
            Literal::String(s) => Value::String(s.clone()),
        }
    }

    fn visit_function_application(&mut self, callee: &Expr, args: &[Expr]) -> Value {
        let func = callee.accept(self);
        match func {
            Value::PrintFn => {
                if args.len() == 1 {
                    let arg_value = args[0].accept(self);
                    match arg_value {
                        Value::String(s) => {
                            writeln!(self.out, "{}", s).expect("Failed to write to stdout");
                            Value::Void
                        }
                        _ => panic!("Expected a String argument to print_ln."),
                    }
                } else {
                    panic!(
                        "Wrong number of args to print_ln function. Expected 1, got: {}",
                        args.len()
                    )
                }
            }
            v => panic!("Cannot call a non-function. Tried to call: {:?}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::{Expr, Stmt};
    use test_with_parameters::*;

    struct FakeWrite {
        written: Vec<String>,
    }

    impl FakeWrite {
        fn new() -> FakeWrite {
            FakeWrite {
                written: Vec::new(),
            }
        }
    }

    impl std::fmt::Write for FakeWrite {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.written.push(s.to_owned());
            Ok(())
        }
    }

    #[test]
    fn empty_program_performs_no_io() {
        let mut write = FakeWrite::new();
        let mut interpreter = Interpreter::new(&mut write);

        let empty_program = UntypedProgram::empty();
        interpreter.interpret(empty_program);

        assert!(write.written.is_empty())
    }

    #[test_with_parameters(
        [ string_to_print, expected_output ]
        [ ""             , ""              ]
        [ "hi wrld"      , "hi wrld"       ]
    )]
    fn println_outputs_arg(string_to_print: &str, expected_output: &str) {
        let (input, output) = (string_to_print.to_owned(), expected_output.to_owned());

        let program = UntypedProgram::with_stmts(vec![Stmt::Expr {
            e: Expr::FunctionApplication {
                callee: Box::new(Expr::identifier("print_ln")),
                args: vec![input.into()],
            },
        }]);

        let mut write = FakeWrite::new();
        let mut interpreter = Interpreter::new(&mut write);

        interpreter.interpret(program);

        assert_eq!(write.written, vec![output, "\n".to_string()])
    }
}
