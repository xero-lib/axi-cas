pub enum Token<'a> {
    Number(&'a [u8]),
    Hex(&'a [u8]),
    Bin(&'a [u8]),
    Oct(&'a [u8]),

    Ident(&'a [u8]),
    
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

    /// Hyperbolic Trig
    Sinh,
    Cosh,
    Tanh,
    Coth,
    Sech,
    Csch,

    /// Inverse Hyperbolic Trig
    Asinh,
    Acosh,
    Atanh,
    Acoth,
    Asech,
    Acsch,

    /// Cardinal Trig
    SincU,
    SincN,

    /// Unary
    Fact,
    Neg,
    Sqrt,
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
    LAnd, /// Logical AND
    Nand,
    Or,
    LOr, /// Logical OR
    Nor,
    Xor,
    Xnor,
    Pderiv,
    Dot,
    Cross,
    Mod,
    Union,
    Percent, // leave as mod?
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
    
    /// Grouping/Misc
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBrack,
    RBrack,
    Uscore,
    Comma,
    Pipe, // for bitwise or logical OR, abs val, or norm
    Amp, // for bitwise or logical AND
    
    Unk(u8),
    Skip,

    EOF,
}