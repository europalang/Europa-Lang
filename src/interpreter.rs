use crate::{
    error::Error,
    nodes::{expr::Expr, stmt::Stmt},
    types::Type, environment::{Environment},
};

type IResult = Result<Type, Error>;

#[derive(Clone, Debug)]
pub struct Interpreter {
    pub nodes: Vec<Stmt>,
    pub environ: Environment
}

impl Interpreter {
    pub fn new(nodes: Vec<Stmt>, environ: Environment) -> Self {
        Self { nodes, environ }
    }

    pub fn init(&self) -> IResult {
        let mut out = Type::Nil;

        for stmt in self.nodes.clone() {
            out = self.eval_stmt(&stmt)?;
        }

        Ok(out)
    }

    fn eval_stmt(&self, stmt: &Stmt) -> IResult {
        match stmt {
            Stmt::Block(stmts) => {
                let mut out = Type::Nil;
                for stmt in stmts {
                    out = self.eval_stmt(stmt)?;
                }
                Ok(out)
            }
            Stmt::ExprStmt(expr) => {
                self.eval_expr(expr)
            },
            Stmt::VarDecl(_) => todo!(),
            Stmt::IfStmt(_, _, _, _) => todo!(),
            Stmt::WhileStmt(_, _) => todo!(),
            Stmt::Break(_) => todo!(),
            Stmt::Continue(_) => todo!(),
            Stmt::Return(_, _) => todo!(),
            Stmt::Function(_, _, _, _) => todo!(),
            Stmt::UseStmt(_, _) => todo!(),
        }
    }

    fn eval_expr(&self, expr: &Expr) -> IResult {
        match expr {
            Expr::Assign(_, _) => todo!(),
            Expr::Binary(_, _, _) => todo!(),
            Expr::Grouping(_) => todo!(),
            Expr::Literal(_) => todo!(),
            Expr::Unary(_, _) => todo!(),
            Expr::Variable(_) => todo!(),
            Expr::Block(_) => todo!(),
            Expr::Logical(_, _, _) => todo!(),
            Expr::Ternary(_, _, _) => todo!(),
            Expr::Call(_, _, _, _) => todo!(),
            Expr::IfExpr(_, _, _, _) => todo!(),
            Expr::Get(_, _, _) => todo!(),
            Expr::Set(_, _, _, _) => todo!(),
            Expr::Prop(_, _) => todo!(),
            Expr::Array(_) => todo!(),
            Expr::Map(_) => todo!(),
            Expr::Range(_, _, _, _) => todo!(),
        }
    }
}
