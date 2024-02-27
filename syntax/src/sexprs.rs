use std::fmt::Display;

use super::lexicon::{Keyword, Reserved, StringConstant, Symbol};

#[derive(Clone, Debug)]
pub enum SpecialConstant {
    // technically the spec only supports unsigned, but there may be solvers that
    // support signed as well, so let's just make it signed.
    Numeral(i64),
    String(StringConstant),
    /* the following items are in the spec, but not implemented yet:
    Hexadecimal(_),
    Binary(_),
    Decimal(_),
    */
}

impl From<i64> for SpecialConstant {
    fn from(value: i64) -> Self {
        Self::Numeral(value)
    }
}

impl From<SpecialConstant> for Sexpr {
    fn from(value: SpecialConstant) -> Self {
        Sexpr::SpecialConstant(value)
    }
}

impl Display for SpecialConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecialConstant::Numeral(num) => num.fmt(f),
            SpecialConstant::String(s) => s.fmt(f),
        }
    }
}

pub enum Sexpr {
    SpecialConstant(SpecialConstant),
    Symbol(Symbol),
    Keyword(Keyword),
    Reserved(Reserved),
    Sequence(Vec<Sexpr>),
}

impl Display for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sexpr::SpecialConstant(sc) => sc.fmt(f),
            Sexpr::Symbol(sym) => sym.fmt(f),
            Sexpr::Keyword(kw) => kw.fmt(f),
            Sexpr::Reserved(r) => r.fmt(f),
            Sexpr::Sequence(seq) => {
                write!(f, "(")?;

                for (i, e) in seq.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{e}")?;
                    } else {
                        write!(f, " {e}")?;
                    }
                }

                write!(f, ")")
            }
        }
    }
}
