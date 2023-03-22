#[derive(Clone, Hash)]
// #[non_exhaustive]
pub enum Expr {
    Identifier(String),

    Comma(Box<CommaExpr>),

    Paren(Box<ParenExpr>),

    Function(Box<FunctionExpr>),

    FunctionCall(Box<FunctionCallExpr>),

    Add(Box<BinaryExpr>),

    /// lhs `&&` rhs
    And(Box<BinaryExpr>),
    /// lhs `||` rhs
    Or(Box<BinaryExpr>),
}

impl std::fmt::Debug for Expr {
    #[cold]
    #[inline(never)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::FunctionCall(x, ..) => std::fmt::Debug::fmt(x, f),

            Expr::And(x) | Expr::Or(x) => {
                let op_name = match self {
                    Self::And(..) => "And",
                    Self::Or(..) => "Or",
                    expr => unreachable!("`And`, `Or` or `Coalesce` expected but gets {:?}", expr),
                };

                f.debug_struct(op_name)
                    .field("lhs", &x.lhs)
                    .field("rhs", &x.rhs)
                    .finish()
            }
            Expr::Identifier(x) => std::fmt::Debug::fmt(x, f),
            Expr::Function(x) => f
                .debug_struct("Function")
                .field("params", &x.params)
                .field("body", &x.body)
                .finish(),
            Expr::Add(x) => f
                .debug_struct("Add")
                .field("operator", &x.operator)
                .field("lhs", &x.lhs)
                .field("rhs", &x.rhs)
                .finish(),
            Expr::Comma(x) => f
                .debug_struct("Comma")
                .field("lhs", &x.lhs)
                .field("rhs", &x.rhs)
                .finish(),
            Expr::Paren(x) => f.debug_struct("Paren").field("expr", &x.expr).finish(),
        }?;
        write!(f, "")
    }
}

#[derive(Clone, Hash)]
pub struct ParenExpr {
    pub expr: Expr,
}

#[derive(Clone, Hash)]
pub struct CommaExpr {
    pub lhs: Expr,
    pub rhs: Expr,
}

#[derive(Clone, Hash)]
pub struct FunctionExpr {
    pub params: Vec<Expr>,
    pub body: Expr,
}

#[derive(Clone, Hash)]
pub struct FunctionCallExpr {
    /// Function name.
    pub name: String,
    /// List of function call argument expressions.
    pub args: Vec<Expr>,
}

impl std::fmt::Debug for FunctionCallExpr {
    #[cold]
    #[inline(never)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("FnCallExpr");
        ds.field("name", &self.name).field("args", &self.args);
        ds.finish()
    }
}

#[derive(Debug, Clone, Hash)]
pub struct BinaryExpr {
    pub operator: String,
    pub lhs: Expr,
    pub rhs: Expr,
}
