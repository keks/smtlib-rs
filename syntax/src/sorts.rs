//! From the spec:
//! > 3.5 Sorts
//! > A major subset of the SMT-LIB language is the language of well-sorted terms, used to represent
//! > logical expressions. Such terms are typed, or sorted in logical terminology; that is, each is
//! > associated with a (unique) sort. The set of sorts consists itself of sort terms. In essence, a sort
//! > term is a sort symbol, a sort parameter, or a sort symbol applied to a sequence of sort terms.
//! > Syntactically, a sort symbol can be either the distinguished symbol Bool or any 〈identier 〉.
//! > A sort parameter can be any 〈symbol 〉 (which in turn, is an 〈identier 〉).
//! > ```
//! > 〈sort〉 ::= 〈identier 〉 | ( 〈identier 〉 〈sort〉+ )
//! > ```

use std::fmt::Display;

use super::{identifiers::Identifier, sexprs::Sexpr};

#[derive(Debug, Clone)]
pub struct Sort {
    identifier: Identifier,
    parameters: Vec<Sort>,
}

impl From<Sort> for Sexpr {
    fn from(value: Sort) -> Self {
        let Sort {
            identifier,
            parameters,
        } = value;
        if parameters.is_empty() {
            identifier.into()
        } else {
            let mut elems: Vec<Sexpr> = vec![identifier.into()];
            elems.extend(parameters.into_iter().map(|param| param.into()));
            Sexpr::Sequence(elems)
        }
    }
}

impl Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.parameters.is_empty() {
            self.identifier.fmt(f)
        } else {
            write!(f, "({id}", id = self.identifier)?;

            for param in &self.parameters {
                write!(f, " {param}")?;
            }

            write!(f, ")")
        }
    }
}
