/* From the spec (p. 22, bottom):
 *
 * The character " can itself occur within a string literal only if duplicated.
 * In other words, after an initial " that starts a literal, a lexer should
 * treat the sequence "" as an escape sequence denoting a single occurrence of
 * " within the literal.
*/
pub fn escape_string(string: &str) -> String {
    string.replace(r#"""#, r#""""#)
}

use std::fmt::Display;

use predicates::{is_printable, is_white_space};

use super::{
    scripts::CommandType,
    sexprs::{Sexpr, SpecialConstant},
    terms::Term,
};

// TODO: impl Display
#[derive(Debug, Clone)]
pub struct InvalidStringError(String);

#[derive(Clone, Debug)]
pub struct StringConstant(String);

impl StringConstant {
    pub fn new(string: String) -> Result<Self, InvalidStringError> {
        if string.chars().all(|c| is_printable(c) || is_white_space(c)) {
            Ok(Self(string))
        } else {
            Err(InvalidStringError(string))
        }
    }

    pub fn new_str(string: &str) -> Result<Self, InvalidStringError> {
        Self::new(string.to_string())
    }

    pub fn new_expect(string: String, expect_msg: &str) -> Self {
        Self::new(string).expect(expect_msg)
    }

    pub fn new_str_expect(string: &str, expect_msg: &str) -> Self {
        Self::new_expect(string.to_string(), expect_msg)
    }

    pub fn string_constant(&self) -> &str {
        &self.0
    }
}
impl From<StringConstant> for SpecialConstant {
    fn from(value: StringConstant) -> Self {
        SpecialConstant::String(value)
    }
}

impl From<StringConstant> for Sexpr {
    fn from(value: StringConstant) -> Self {
        Sexpr::SpecialConstant(value.into())
    }
}

impl From<i64> for Sexpr {
    fn from(value: i64) -> Self {
        Sexpr::SpecialConstant(value.into())
    }
}

impl From<i64> for Term {
    fn from(value: i64) -> Self {
        Term::SpecialConstant(value.into())
    }
}

impl Display for StringConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#""{}""#, escape_string(&self.0))
    }
}

pub enum Reserved {
    Binary,
    Decimal,
    Hexadecimal,
    Numeral,
    String,
    Underscore,
    Bang,
    As,
    Let,
    Exists,
    Forall,
    Match,
    Par,
    Command(CommandType),
}

impl Display for Reserved {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reserved::Binary => write!(f, "BINARY"),
            Reserved::Decimal => write!(f, "DECIMAL"),
            Reserved::Hexadecimal => write!(f, "HEXADECIMAL"),
            Reserved::Numeral => write!(f, "NUMERAL"),
            Reserved::String => write!(f, "STRING"),
            Reserved::Underscore => write!(f, "_"),
            Reserved::Bang => write!(f, "!"),
            Reserved::As => write!(f, "as"),
            Reserved::Let => write!(f, "let"),
            Reserved::Exists => write!(f, "exists"),
            Reserved::Forall => write!(f, "forall"),
            Reserved::Match => write!(f, "match"),
            Reserved::Par => write!(f, "par"),
            Reserved::Command(cmd) => match cmd {
                CommandType::Assert => write!(f, "assert"),
                CommandType::CheckSat => write!(f, "check-sat"),
                CommandType::CheckSatAssuming => write!(f, "check-sat-assuming"),
                CommandType::DeclareConst => write!(f, "declare-const"),
                CommandType::DeclareDatatype => write!(f, "declare-datatype"),
                CommandType::DeclareDatatypes => write!(f, "declare-datatypes"),
                CommandType::DeclareFun => write!(f, "declare-fun"),
                CommandType::DeclareSort => write!(f, "declare-sort"),
                CommandType::DefineFun => write!(f, "define-fun"),
                CommandType::DefineFunRec => write!(f, "define-fun-rec"),
                CommandType::DefineFunsRec => write!(f, "define-funs-rec"),
                CommandType::DefineSort => write!(f, "define_sort"),
                CommandType::Echo => write!(f, "echo"),
                CommandType::Exit => write!(f, "exit"),
                CommandType::GetAssertions => write!(f, "get-assertions"),
                CommandType::GetAssignment => write!(f, "get-assignment"),
                CommandType::GetModel => write!(f, "get-model"),
                CommandType::GetOption => write!(f, "get-option"),
                CommandType::GetProof => write!(f, "get-proof"),
                CommandType::GetUnsatAssumptions => write!(f, "get-unsat-assumptions"),
                CommandType::GetUnsatCore => write!(f, "get-unsat-core"),
                CommandType::GetValue => write!(f, "get-value"),
                CommandType::Pop => write!(f, "pop"),
                CommandType::Push => write!(f, "push"),
                CommandType::Reset => write!(f, "reset"),
                CommandType::ResetAssertions => write!(f, "reset-assertions"),
                CommandType::SetLogic => write!(f, "set-logic"),
            },
        }
    }
}

impl From<Reserved> for Sexpr {
    fn from(value: Reserved) -> Self {
        Sexpr::Reserved(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Symbol {
    symbol: String,
    is_quoted: bool,
}

impl From<Symbol> for Sexpr {
    fn from(value: Symbol) -> Self {
        Sexpr::Symbol(value)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_quoted() {
            write!(f, "|")?;
            write!(f, "{}", self.symbol)?;
            write!(f, "|")
        } else {
            write!(f, "{}", self.symbol)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidSymbolError(String);

impl Symbol {
    pub fn new(symbol: String) -> Result<Symbol, InvalidSymbolError> {
        if symbol.contains('|') || symbol.contains('\\') {
            Err(InvalidSymbolError(symbol))
        } else {
            let is_quoted = !predicates::is_simple_symbol(&symbol);

            Ok(Symbol { symbol, is_quoted })
        }
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn is_quoted(&self) -> bool {
        self.is_quoted
    }
}

#[derive(Clone, Debug)]
pub struct Keyword(String);

impl Keyword {
    pub fn keyword(&self) -> &str {
        &self.0
    }
}

impl From<Keyword> for Sexpr {
    fn from(value: Keyword) -> Self {
        Sexpr::Keyword(value)
    }
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ":{}", self.0)
    }
}

pub mod predicates {
    const RESERVED: [&str; 13] = [
        "!",
        "BINARY",
        "DECIMAL",
        "HEXADECIMAL",
        "NUMERAL",
        "STRING",
        "_",
        "as",
        "exists",
        "forall",
        "let",
        "match",
        "par",
    ];

    pub fn is_white_space(c: char) -> bool {
        match c {
            '\t' | '\r' | '\n' | ' ' => true,
            _ => false,
        }
    }

    pub fn is_printable(c: char) -> bool {
        match c {
            '\0'..='\x1f' | '\x7f' => false,
            _ => true,
        }
    }

    pub fn is_digit(c: char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }

    pub fn is_letter(c: char) -> bool {
        match c {
            'A'..='Z' | 'a'..='z' => true,
            _ => false,
        }
    }

    pub fn is_numeral(s: &str) -> bool {
        if s == "0" {
            return true;
        }

        for (i, c) in s.chars().enumerate() {
            match (i, c) {
                // only digits allowed
                (_, c) if !is_digit(c) => return false,

                // no leading zeroes allowed
                (0, '0') => return false,

                // accept the rest
                _ => {}
            }
        }

        true
    }

    pub fn is_simple_symbol(s: &str) -> bool {
        /* A simple symbol is any non-empty sequence of elements of
         * 〈letter 〉 and 〈digit〉 and the characters
         *      ~ ! @ $ % ^ & * _ - + = < > . ? /
         * that does not start with a digit and is not a reserved word.
         * */

        let legal_nonletters = "~!@$%^&*_-+=<>.?/";

        for (i, c) in s.chars().enumerate() {
            match (i, c) {
                // no leading digits allowed
                (0, c) if is_digit(c) => return false,

                // a non-empty sequence of letters, digits, and the characters (...)
                (_, c) if is_letter(c) || is_digit(c) || legal_nonletters.contains(c) => {}

                // anything else is forbidden
                _ => return false,
            }
        }

        !s.is_empty() && !RESERVED.contains(&s)
    }
}
