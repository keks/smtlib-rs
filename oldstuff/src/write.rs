use crate::syntax::{
    lexicon::{Keyword, StringConstant, Symbol},
    sexprs::{Sexpr, SpecialConstant},
};

pub trait Writable {
    fn write_to<W: Write>(&self, w: &mut W) -> Result<W::Success, W::Error>;
}

pub trait Monoid {
    fn join(self, other: Self) -> Self;
}

impl Monoid for () {
    fn join(self, other: Self) -> Self {
        ()
    }
}

pub trait Write {
    type Success: Monoid;
    type Error;

    fn write_str(&mut self, word: &str) -> Result<Self::Success, Self::Error>;
}

pub mod io {
    pub struct Writer<W: std::io::Write>(W);

    impl<W: std::io::Write> super::Write for Writer<W> {
        type Success = ();

        type Error = std::io::Error;

        fn write_str(&mut self, word: &str) -> Result<Self::Success, Self::Error> {
            write!(&mut self.0, "{word}")
        }
    }
}

pub mod fmt {
    pub struct Writer<'a>(core::fmt::Formatter<'a>);

    impl<'a> super::Write for Writer<'a> {
        type Success = ();

        type Error = core::fmt::Error;

        fn write_str(&mut self, word: &str) -> Result<Self::Success, Self::Error> {
            write!(&mut self.0, "{word}")
        }
    }
}

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

impl Writable for StringConstant {
    fn write_to<W: Write>(&self, w: &mut W) -> Result<W::Success, W::Error> {
        w.write_str(&format!(r#""{}""#, escape_string(self.string_constant())))
    }
}

impl Writable for Symbol {
    fn write_to<W: Write>(&self, w: &mut W) -> Result<W::Success, W::Error> {
        if self.is_quoted() {
            let succ1 = w.write_str("|")?;
            let succ2 = w.write_str(self.symbol())?;
            w.write_str("|").map(|succ3| succ1.join(succ2).join(succ3))
        } else {
            w.write_str(self.symbol())
        }
    }
}

impl Writable for Keyword {
    fn write_to<W: Write>(&self, w: &mut W) -> Result<W::Success, W::Error> {
        let succ = w.write_str(":")?;
        w.write_str(self.keyword()).map(|s| succ.join(s))
    }
}

impl Writable for SpecialConstant {
    fn write_to<W: Write>(&self, w: &mut W) -> Result<W::Success, W::Error> {
        match self {
            SpecialConstant::Numeral(num) => w.write_str(&num.to_string()),
            SpecialConstant::String(string) => string.write_to(w),
        }
    }
}
