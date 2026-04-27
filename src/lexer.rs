use crate::Token;

pub const MAX_BUILTIN_FUNC_NAME_LEN: usize = 9; // "intersect", probably change this 

pub fn lex_ident<'a>(data: &'a [u8]) -> usize {
    let mut offset: usize = 1;

    loop {
        if offset >= data.len() {
            return offset;
        }

        match data[offset] {
            b'a'..=b'z' | b'A'..=b'Z' => offset += 1,
            _ => return offset
        }
    }
}

pub fn lex_hex<'a>(data: &'a [u8]) -> usize {
    let mut offset = 2;
    
    loop {
        if offset >= data.len() {
            return offset;
        }

        match data[offset] {
            b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => offset += 1,
            _ => return offset
        }
    }
}

pub fn lex_bin<'a>(data: &'a [u8]) -> usize {
    let mut offset = 2;
    
    loop {
        if offset >= data.len() {
            return offset;
        }

        match data[offset] {
            b'0' | b'1' => offset += 1,
            _ => return offset
        }
    }
}

pub fn lex_oct<'a>(data: &'a [u8]) -> usize {
    let mut offset = 2;
    
    loop {
        if offset >= data.len() {
            return offset;
        }

        match data[offset] {
            b'0'..=b'7' => offset += 1,
            _ => return offset
        }
    }
}

pub fn lex_num<'a>(data: &'a [u8]) -> usize {
    let mut offset = 1;
    let mut seen_dot = false;

    loop {
        if offset >= data.len() {
            return offset;
        }

        match data[offset] {
            b'0'..=b'9' => offset += 1,
            b'.' => if seen_dot { return offset } else { seen_dot = true; offset += 1 },
            _ => return offset
        }
    }
}

pub fn lex_combo<'a>(data: &'a [u8]) -> (Token<'a>, usize) {
    match data {
        b"+-" => (Token::Pm, 2),
        b"-+" => (Token::Mp, 2),
        b"!=" => (Token::Neq, 2),
        b">=" => (Token::Geq, 2),
        b"<=" => (Token::Leq, 2),
        b":=" => (Token::Def, 2),
        b"||" => (Token::LOr, 2),
        b"&&" => (Token::LAnd, 2),
        b".." => (Token::Range, 2),
        b"~^" | b"!^" => (Token::Xnor, 2),

        _ => (Token::Skip, 0)
    }
}

// pub fn lex_symbol() {
//                 /// Unary
//                 "!" => Token::Fact,
//                 "~" => Token::Neg,

//                 /// Binary
//                 "+" => Token::Add,
//                 "-" => Token::Sub, // handle grouping for neg in parser
//                 "*" => Token::Mul, // handle grouping for dot product in parser
//                 "/" => Token::Div, // handle fraction creation determination in parser
//                 "^" => Token::Pow, 
// }

pub struct Lexer<'a> {
    data: &'a [u8],
    cursor: usize,
    // next: Token<'a>
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Self {
        Self { data: data.as_bytes(), cursor: 0 }
    }

    pub fn peek(&self) -> (Token<'a>, usize) {
        if self.cursor >= self.data.len() {
            return (Token::EOF, 0);
        }

        // check if function
        let current = self.data[self.cursor];
        
        if current == b'0' && self.cursor + 1 < self.data.len() {
            match self.data[self.cursor + 1] as char {
                'x' => {
                    let len = lex_hex(&self.data[self.cursor..]);
                    if len != 2 {
                        return (Token::Hex(&self.data[self.cursor..len]), len);
                    }
                },
                'b' => {
                    let len = lex_bin(&self.data[self.cursor..]);
                    if len != 2 {
                        return (Token::Bin(&self.data[self.cursor..len]), len);
                    }
                },
                'o' => {
                    let len = lex_oct(&self.data[self.cursor..]);
                    if len != 2 {
                        return (Token::Oct(&self.data[self.cursor..len]), len);
                    }
                },
                _ => {}
            };
        }



        if current.is_ascii_alphabetic() {
            let offset = lex_ident(&self.data[self.cursor..]);
            let slice = &self.data[self.cursor..self.cursor + offset];
            if offset > MAX_BUILTIN_FUNC_NAME_LEN {
                return (Token::Ident(slice), offset)
            }

            let token = match slice {
                // Trig
                b"sin" => Token::Sin,
                b"cos" => Token::Cos,
                b"tan" => Token::Tan,
                b"cot" => Token::Cot,
                b"sec" => Token::Sec,
                b"csc" => Token::Csc,

                // Inverse Trig (handle ^-1 notation in parser)
                b"asin" | b"arcsin" => Token::Asin,
                b"acos" | b"arccos" => Token::Acos,
                b"atan" | b"arctan" => Token::Atan,
                b"acot" | b"arccot" => Token::Acot,
                b"asec" | b"arcsec" => Token::Asec,

                // Hyperbolic Trig
                b"sinh" => Token::Sinh,
                b"cosh" => Token::Cosh,
                b"tanh" => Token::Tanh,
                b"coth" => Token::Coth,
                b"sech" => Token::Sech,
                b"csch" => Token::Csch,

                // Inverse Hyperbolic Trig
                b"asinh" | b"arcsinh" => Token::Asinh,
                b"acosh" | b"arccosh" => Token::Acosh,
                b"atanh" | b"arctanh" => Token::Atanh,
                b"acoth" | b"arccoth" => Token::Acoth,
                b"asech" | b"arcsech" => Token::Asech,
                b"acsch" | b"arccsch" => Token::Acsch,

                // Cardinal Trig
                b"sinc"  | b"sincu"     => Token::SincU,
                b"sincn" | b"sinc_norm" => Token::SincN,
                
                // Unary
                b"ln" => Token::Ln,

                // Binary
                b"nrt" | b"nthrt" | b"nthroot" | b"nroot" => Token::Nrt,
                b"AND" => Token::And,
                b"NAND" => Token::Nand,
                b"OR" => Token::Or,
                b"NOR" => Token::Nor,
                b"XOR" => Token::Xor,
                b"XNOR" => Token::Xnor,
                b"partial" | b"pderiv" => Token::Pderiv,
                b"dot" | b"dotp" | b"dp" | b"dotprod" | b"iprod" | b"innerprod" => Token::Dot,
                b"cross" | b"crossp" | b"crossprod" => Token::Cross,
                b"mod" | b"modulo" => Token::Mod,
                b"union" => Token::Union,
                b"intersect" | b"intersection" => Token::Intersect,

                // Ternary
                b"lim" | b"limit" => Token::Lim,

                // 4-ary
                b"sum" | b"Sigma" => Token::Sum,
                b"prod" | b"Pi" => Token::Prod,

                // Variable-argument
                b"int" | b"integrate" | b"integral" => Token::Integral,
                b"log" => Token::Log,

                // Relation
                b"def" | b"let" | b"var" | b"sto" => Token::Def,
                b"pi" => Token::Pi,
                b"e" => Token::E,
                b"i" => Token::I,
                b"inf" | b"infty" | b"infinity" => Token::Inf,
                b"deg" | b"degree" => Token::Deg,
                b"rad" | b"radian" => Token::Rad,
                b"Ans" | b"ans" => Token::Ans,

                _ => Token::Ident(slice)
            };

            return (token, offset);
        }
        
        if current.is_ascii_digit() {
            let len = lex_num(&self.data[self.cursor..]);
            return (Token::Number(&self.data[self.cursor..self.cursor + len]), len)
        }
        
        if self.cursor + 2 < self.data.len() && current.is_ascii_punctuation() {
            let (token, size) = lex_combo(&self.data[self.cursor..self.cursor + 2]);
            if size != 0 {
                return (token, size);
            }
        }

        let token = match current {
            b'+' => Token::Add,
            b'-' => Token::Sub,
            b'*' => Token::Mul,
            b'/' => Token::Div,
            b'~' => Token::Neg,
            b'%' => Token::Percent,
            b'!' => Token::Fact,
            b'<' => Token::Lt,
            b'>' => Token::Gt,
            b'=' => Token::Eq,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'[' => Token::LBrack,
            b']' => Token::RBrack,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'_' => Token::Uscore,
            b',' => Token::Comma,
            b'|' => Token::Pipe,
            b'&' => Token::Amp,
            _ => Token::Unk(current)
        };

        (token, 1)
    }

    pub fn next(&mut self) -> Token<'a> {
        let (token, size) = self.peek();
        self.cursor += size;
        token
    }
}