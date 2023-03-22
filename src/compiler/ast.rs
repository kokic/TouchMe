

#[derive(Clone, Hash)]
// #[non_exhaustive]
pub enum Expr {
    /// func `(` expr `,` ... `)`
    FnCall(Box<FnCallExpr>),
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
            Self::FnCall(x, ..) => std::fmt::Debug::fmt(x, f),

            Self::And(x) | Self::Or(x) => {
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
        }?;

        write!(f, "@ ")
    }
}

#[derive(Clone, Hash)]
pub struct FnCallExpr {
    /// Function name.
    pub name: String,
    /// List of function call argument expressions.
    pub args: Vec<Expr>,
}

impl std::fmt::Debug for FnCallExpr {
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
    /// LHS expression.
    pub lhs: Expr,
    /// RHS expression.
    pub rhs: Expr,
}
