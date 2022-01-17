use token::*;

#[derive(Debug, PartialEq)]
pub struct UntypedProgram {
    pub stmts: Vec<Stmt>,
}

impl UntypedProgram {
    pub fn pretty_printed(&self) -> String {
        format!("#{:#?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr { e: Expr },
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Identifier { name: Token },
    Literal { l: Literal },
    FunctionApplication { callee: Box<Expr>, args: Vec<Expr> },
}

impl<T: Into<Literal>> From<T> for Expr {
    fn from(l: T) -> Expr {
        Expr::Literal { l: l.into() }
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
}

impl Literal {
    pub fn new<T: Into<Literal>>(t: T) -> Literal {
        t.into()
    }
}

impl<T: Into<String>> From<T> for Literal {
    fn from(t: T) -> Literal {
        Literal::String(t.into())
    }
}
