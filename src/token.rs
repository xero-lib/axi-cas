use core::fmt::Display;

#[derive(PartialEq, Debug)]
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
    Percent, // check params in parser
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

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Token::Abs => "abs",
            Token::Acos => "arccos",
            Token::Acosh => "arccosh",
            Token::Acot => "arccot",
            Token::Acoth => "arccoth",
            Token::Acsc => "arccsc",
            Token::Acsch => "arccsch",
            Token::Add => "+",
            Token::Amp => "&",
            Token::And => "AND",
            Token::Ans => "Ans",
            Token::Asec => "arcsec",
            Token::Asech => "arcsech",
            Token::Asin => "arcsin",
            Token::Asinh => "arcsinh",
            Token::Atan => "arctan",
            Token::Atanh => "arctanh",
            Token::Bin(b) => str::from_utf8(b).unwrap(),
            Token::Comma => ",",
            Token::Cos => "cos",
            Token::Cosh => "cosh",
            Token::Cot => "cot",
            Token::Coth => "coth",
            Token::Cross => "cross",
            Token::Csc => "csc",
            Token::Csch => "csch",
            Token::Def => ":=",
            Token::Deg => "deg",
            Token::Deriv => "d/dx",
            Token::Div => "/",
            Token::Dot => "dp",
            Token::E => "e",
            Token::EOF => "EOF",
            Token::Eq => "=",
            Token::Fact => "!",
            Token::Gamma => "Gamma",
            Token::Geq => ">=",
            Token::Grad => "del",
            Token::Gt => ">",
            Token::Hex(h) => str::from_utf8(h).unwrap(),
            Token::I => "i",
            Token::Ident(v) => str::from_utf8(v).unwrap(),
            Token::Inf => "infty",
            Token::Integral => "int",
            Token::Intersect => "intersect",
            Token::LAnd => "&&",
            Token::LBrace => "{",
            Token::LBrack => "[",
            Token::LOr => "||",
            Token::LParen => "(",
            Token::Leq => "<=",
            Token::Lim => "lim",
            Token::Ln => "ln",
            Token::Log => "log",
            Token::Lt => "<",
            Token::Mod => "MOD",
            Token::Mp => "-+",
            Token::Mul => "*",
            Token::Nand => "NAND",
            Token::Neg => "~",
            Token::Neq => "!=",
            Token::Nor => "NOR",
            Token::Norm => "norm",
            Token::Nrt => "nth-root",
            Token::Number(n) => str::from_utf8(n).unwrap(),
            Token::Oct(o) => str::from_utf8(o).unwrap(),
            Token::Or => "OR",
            Token::Pderiv => "partial",
            Token::Percent => "%",
            Token::Pi => "pi",
            Token::Pipe => "|",
            Token::Pm => "+-",
            Token::Pow => "^",
            Token::Prod => "Pi",
            Token::RBrace => "}",
            Token::RBrack => "]",
            Token::RParen => ")",
            Token::Rad => "rad",
            Token::Range => "..",
            Token::Sec => "sec",
            Token::Sech => "sech",
            Token::Sin => "sin",
            Token::SincN => "sincn",
            Token::SincU => "sincu",
            Token::Sinh => "sinh",
            Token::Skip => "",
            Token::Sqrt => "sqrt",
            Token::Sub => "-",
            Token::Sum => "Sigma",
            Token::Tan => "tan",
            Token::Tanh => "tanh",
            Token::Union => "union",
            Token::Unk(b) => str::from_utf8(core::slice::from_ref(b)).unwrap(),
            Token::Uscore => "_",
            Token::Xnor => "XNOR",
            Token::Xor => "XOR",
            Token::Zeta => "zeta"
        })
    }
}
