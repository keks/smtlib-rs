//! From the spec:
//! > 3.3 Identifiers
//! >
//! > Identifiers are used mostly as function and sort symbols. When defining certain SMT-LIB
//! > theories it is convenient to have indexed identifiers as well. Instead of having a special token
//! > syntax for that, indexed identifiers are defined more systematically as the application of the
//! > reserved word _ to a symbol and one or more indices. Indices can be numerals or symbols.(8)
//! > ```
//! >  〈index 〉      ::= 〈numeral 〉 | 〈symbol 〉
//! >  〈identifier 〉 ::= 〈symbol 〉 | ( _ 〈symbol 〉 〈index 〉+ )
//! > ```

use std::fmt::Display;

use super::{
    lexicon::{Reserved, Symbol},
    sexprs::{Sexpr, SpecialConstant::Numeral},
};

#[derive(Debug, Clone)]
pub enum Index {
    Numeral(i64),
    Symbol(Symbol),
}

impl From<Index> for Sexpr {
    fn from(value: Index) -> Self {
        match value {
            Index::Numeral(num) => Sexpr::SpecialConstant(Numeral(num)),
            Index::Symbol(sym) => Sexpr::Symbol(sym),
        }
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Index::Numeral(num) => num.fmt(f),
            Index::Symbol(sym) => sym.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    symbol: Symbol,
    indexes: Vec<Index>,
}

impl From<Identifier> for Sexpr {
    fn from(value: Identifier) -> Self {
        let Identifier { symbol, indexes } = value;
        if indexes.is_empty() {
            Sexpr::Symbol(symbol)
        } else {
            let mut elems = vec![Sexpr::Reserved(Reserved::Underscore), Sexpr::Symbol(symbol)];

            elems.extend(indexes.into_iter().map(|index| index.into()));

            Sexpr::Sequence(elems)
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.indexes.is_empty() {
            self.symbol.fmt(f)
        } else {
            write!(f, "(_ {sym}", sym = self.symbol)?;

            for index in &self.indexes {
                write!(f, " {index}")?;
            }

            write!(f, ")")
        }
    }
}
