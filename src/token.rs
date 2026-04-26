pub enum Token<'a> {
    Start,

    Number(&'a str),
    Hex(&'a str),
    Bin(&'a str),
    Oct(&'a str),

    Ident(&'a str),
    
    /// Trig
    Sin,
    Cos,
    Tan,
    Cot,
    Sec,
    Csc,

    /// Inverse Trig
    Asin,
    Acos,
    Atan,
    Acot,
    Asec,
    Acsc,

    /// Unary
    Fact,
    Neg,
    Sqrt,
    Nsum,
    Nprod,
    Abs,
    Norm,
    Ln,
    Gamma,
    Zeta,
    Grad,

    /// Binary
    Add,
    Sub,
    Mul,
    Div,
    Pow, // remember to track E for sci notation
    Nrt,
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Xnor,
    Pderiv,
    Dot,
    Cross,
    Mod,
    Union,
    Intersect,

    

    /// Ternary
    Lim,

    /// 4-ary
    Sum,
    Prod,

    /// Variable-argument
    Integral, // definite or indefinite
    Deriv, // derivative at points or symbolic
    Log,
    Pm, // plus-minus
    Mp, // minus-plus
    Range,
    
    /// Relation
    Eq,
    Lt,
    Gt,
    Geq,
    Leq,
    Neq,
    Def,

    /// Constants/Symbols/Reserved
    Pi,
    E,
    I,
    Inf,
    Deg,
    Rad,
    Ans,
    
    /// Grouping
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBrack,
    RBrack,
    Uscore,
    Comma,

    EOF,
}