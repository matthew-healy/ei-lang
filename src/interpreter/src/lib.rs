use std::{cell::RefCell, collections::HashMap, io::Write, rc::Rc};

use ast::{Expr, ExprVisitor, Literal, UntypedProgram};
use token::Token;

struct NativeFn {
    body: Box<dyn FnMut(&[Value]) -> Value>,
}

struct Globals {
    ns: HashMap<String, NativeFn>,
}

impl<'program> Globals {
    fn new() -> Globals {
        Globals { ns: HashMap::new() }
    }

    fn add<S: Into<String>>(&mut self, k: S, v: NativeFn) {
        self.ns.insert(k.into(), v);
    }

    fn call(&mut self, name: &str, args: &[Value]) -> Value {
        let func = self
            .ns
            .get_mut(name)
            .expect(&format!("No such function {}", name));
        (func.body)(args)
    }
}

#[derive(Debug)]
enum Value {
    GlobalFn(String),
    String(String),
    Void,
}

struct Interpreter {
    globals: Globals,
}

impl Interpreter {
    fn new<W: Write + 'static>(write: Rc<RefCell<W>>) -> Interpreter {
        let mut globals = Globals::new();

        globals.add(
            "print_ln",
            NativeFn {
                body: Box::new(move |args| {
                    let s = match args {
                        [Value::String(s)] => s,
                        _ => panic!("Incorrect arguments for print_ln"),
                    };
                    writeln!(write.borrow_mut(), "{}", s).expect("Oh no!");
                    Value::Void
                }),
            },
        );

        Interpreter { globals }
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

impl ExprVisitor<Value> for Interpreter {
    fn visit_identifier(&mut self, name: &Token) -> Value {
        Value::GlobalFn(name.lexeme.clone())
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
            Value::GlobalFn(name) => {
                let arg_vals: Vec<Value> = args.iter().map(|e| e.accept(self)).collect();
                self.globals.call(&name, &arg_vals)
            }
            v => panic!("Cannot call a non-function. Tried to call: {:?}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

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

    impl Write for FakeWrite {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let s = std::str::from_utf8(buf).unwrap();
            self.written.push(s.to_owned());
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            // Ok(())
            todo!()
        }
    }

    #[test]
    fn empty_program_performs_no_io() {
        let write = Rc::new(RefCell::new(FakeWrite::new()));
        let our_write = Rc::clone(&write);

        let mut interpreter = Interpreter::new(write);

        let empty_program = UntypedProgram::empty();
        interpreter.interpret(empty_program);

        assert!(our_write.borrow().written.is_empty())
    }

    #[test_with_parameters(
        [ string_to_print, expected_output       ]
        [ ""             , &["\n"]            ]
        [ "hi wrld"      , &["hi wrld", "\n"] ]
    )]
    fn println_outputs_arg(string_to_print: &str, expected_output: &[&str]) {
        let (input, output) = (string_to_print.to_owned(), expected_output.to_owned());

        let program = UntypedProgram::with_stmts(vec![Stmt::Expr {
            e: Expr::FunctionApplication {
                callee: Box::new(Expr::identifier("print_ln")),
                args: vec![input.into()],
            },
        }]);

        let write = Rc::new(RefCell::new(FakeWrite::new()));
        let our_write = Rc::clone(&write);

        let mut interpreter = Interpreter::new(write);

        interpreter.interpret(program);

        let written = our_write.borrow().written.clone();
        assert_eq!(written, output)
    }
}
