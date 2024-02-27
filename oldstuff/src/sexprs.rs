extern crate alloc;

use crate::syntax::{Constant, Keyword, Symbol};

use self::builder::SExprBuilder;

pub mod builder;
pub mod fmt;
pub mod io;

#[derive(Clone, Debug)]
pub enum SExpr {
    Constant(Constant),
    Symbol(Symbol),
    Keyword(Keyword),
    Word(String),
    List(Vec<SExpr>),
}

impl Write for SExpr {
    type Error<W: Writer> = W::Error;

    fn write<W: Writer>(&self, w: W) -> Result<W::Next, W::Error> {
        match self {
            SExpr::Constant(constant) => w.write_const(constant),
            SExpr::Symbol(symbol) => w.write_symbol(symbol),
            SExpr::Keyword(_) => unreachable!(),
            SExpr::Word(word) => w.write_word(word),
            SExpr::List(list) => {
                let mut inner = w.enter()?;
                for item in list {
                    inner = item.write(inner)?;
                }
                inner.leave()
            }
        }
    }
}
pub trait ToSExpr {
    fn to_sexpr(&self) -> SExpr;
}

pub trait WriteError<E>: std::fmt::Debug {}

pub trait Write {
    type Error<W: Writer>: WriteError<W::Error>;

    fn write<W: Writer>(&self, w: W) -> Result<W::Next, Self::Error<W>>;
}

pub trait ChildWriter: Writer<Next = Self> {
    type Parent;

    fn leave(self) -> Result<Self::Parent, Self::Error>;
}

pub trait Writer: Sized {
    type Error: WriteError<Self::Error>;
    type Next;
    type Child: ChildWriter<Error = Self::Error, Parent = Self::Next>;

    fn enter(self) -> Result<Self::Child, Self::Error>;
    fn write_word(self, word: &str) -> Result<Self::Next, Self::Error>;

    fn write_const(self, constant: &Constant) -> Result<Self::Next, Self::Error> {
        constant.write(self)
    }

    fn write_symbol(self, symbol: &Symbol) -> Result<Self::Next, Self::Error> {
        symbol.write(self)
    }

    fn write_keyword(self, keyword: &Keyword) -> Result<Self::Next, Self::Error> {
        keyword.write(self)
    }
}

impl<T: crate::BlanketMarker + Write> ToSExpr for T {
    fn to_sexpr(&self) -> SExpr {
        let builder = SExprBuilder::new();
        self.write(builder).unwrap()
    }
}

impl SExpr {
    pub fn word(word: impl ToString) -> Self {
        Self::Word(word.to_string())
    }

    pub fn list(list: impl Iterator<Item = SExpr>) -> Self {
        Self::List(list.collect())
    }
}

impl FromIterator<SExpr> for SExpr {
    fn from_iter<T: IntoIterator<Item = SExpr>>(iter: T) -> Self {
        Self::list(iter.into_iter())
    }
}

impl<'a> Writer for &mut std::fmt::Formatter<'a> {
    type Error = std::fmt::Error;

    type Next = Self;

    type Child = Self;

    fn enter(self) -> Result<Self::Child, Self::Error> {
        write!(self, "(")?;
        Ok(self)
    }

    fn write_word(self, word: &str) -> Result<Self::Next, Self::Error> {
        write!(self, "{word}")?;
        Ok(self)
    }
}

impl<'a> ChildWriter for &mut std::fmt::Formatter<'a> {
    type Parent = Self;

    fn leave(self) -> Result<Self::Parent, Self::Error> {
        write!(self, ")")?;
        Ok(self)
    }
}

impl core::fmt::Display for SExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write(f).map(|_| ())
    }
}
impl Write for crate::syntax::sexprs::SpecialConstant {
    type Error<W: Writer> = W::Error;

    fn write<W: Writer>(&self, w: W) -> Result<W::Next, Self::Error<W>> {
        let raw = match self {
            Constant::Numeral(num) => format!("{num}"),
            Constant::String(string) => format!("{}", crate::syntax::escape_string(string)),
        };

        w.write_word(&raw)
    }
}

impl Write for crate::syntax::lexicon::Symbol {
    type Error<W: Writer> = W::Error;

    fn write<W: Writer>(&self, w: W) -> Result<W::Next, Self::Error<W>> {
        let Self { symbol, is_quoted } = self;
        if *is_quoted {
            w.write_word(&format!("|{symbol}|"))
        } else {
            w.write_word(&format!("{symbol}"))
        }
    }
}

impl Write for crate::syntax::lexicon::Keyword {
    type Error<W: Writer> = W::Error;

    fn write<W: Writer>(&self, w: W) -> Result<W::Next, Self::Error<W>> {
        w.write_word(&format!(":{kw}", kw = self.0))
    }
}
