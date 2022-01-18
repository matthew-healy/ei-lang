use token::*;

#[derive(Debug, PartialEq)]
pub struct UntypedProgram {
    pub stmts: Vec<Stmt>,
}

impl UntypedProgram {
    pub fn empty() -> UntypedProgram {
        UntypedProgram { stmts: Vec::new() }
    }

    pub fn with_stmts(stmts: Vec<Stmt>) -> UntypedProgram {
        UntypedProgram { stmts }
    }

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

impl Expr {
    pub fn identifier<S: Into<String>>(s: S) -> Expr {
        Expr::Identifier {
            name: Token::identifier(s.into()),
        }
    }
}

// TODO: this is a bad idea, since at the callsite
// Expr::from("a") could reasonably be expected to produce
// an identifier rather than a literal.
impl<T: Into<Literal>> From<T> for Expr {
    fn from(l: T) -> Expr {
        Expr::Literal { l: l.into() }
    }
}

pub trait ExprVisitor<V> {
    fn visit_identifier(&mut self, name: &Token) -> V;
    fn visit_literal(&mut self, l: &Literal) -> V;
    fn visit_function_application(&mut self, callee: &Expr, args: &[Expr]) -> V;
}

impl Expr {
    pub fn accept<Val, Visitor: ExprVisitor<Val>>(&self, visitor: &mut Visitor) -> Val {
        match self {
            Expr::Identifier { name } => visitor.visit_identifier(name),
            Expr::Literal { l } => visitor.visit_literal(l),
            Expr::FunctionApplication { callee, args } => {
                visitor.visit_function_application(callee, args)
            }
        }
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
