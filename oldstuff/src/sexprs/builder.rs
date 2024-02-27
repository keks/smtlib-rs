use super::{ChildWriter, SExpr, WriteError, Writer};

#[derive(Debug, Clone)]
pub enum Error {
    MoreThanOneWriteToSingleWord,
    EmptyLeave,
}

impl WriteError<Self> for Error {}

trait Depth {
    fn depth() -> usize;
}

pub struct SExprBuilder<T>(Option<SExpr>, T);

impl SExprBuilder<()> {
    pub fn new() -> Self {
        Self(None, ())
    }

    pub fn finish(self) -> Option<SExpr> {
        self.0
    }
}

impl Depth for () {
    fn depth() -> usize {
        0
    }
}

impl<T: Depth> Depth for SExprBuilder<T> {
    fn depth() -> usize {
        1 + T::depth()
    }
}

impl Writer for SExprBuilder<()> {
    type Error = Error;
    type Child = SExprBuilder<Self>;
    type Next = SExpr;

    fn enter(self) -> Result<Self::Child, Self::Error> {
        match self.0 {
            None | Some(SExpr::List(_)) => Ok(SExprBuilder(None, self)),
            Some(SExpr::Word(_) | SExpr::Constant(_) | SExpr::Symbol(_) | SExpr::Keyword(_)) => {
                Err(Error::MoreThanOneWriteToSingleWord)
            }
        }
    }

    fn write_word(self, word: &str) -> Result<SExpr, Self::Error> {
        Ok(SExpr::word(word))
    }
}

impl<T: Depth> Writer for SExprBuilder<SExprBuilder<T>> {
    type Error = Error;
    type Child = SExprBuilder<Self>;
    type Next = Self;

    fn enter(self) -> Result<Self::Child, Self::Error> {
        match self.0 {
            None | Some(SExpr::List(_)) => Ok(SExprBuilder(None, self)),
            Some(SExpr::Word(_) | SExpr::Constant(_) | SExpr::Symbol(_) | SExpr::Keyword(_)) => {
                Err(Error::MoreThanOneWriteToSingleWord)
            }
        }
    }
    fn write_word(mut self, word: &str) -> Result<Self, Self::Error> {
        let word = SExpr::word(word);

        match &mut self.0 {
            Some(SExpr::List(list)) => list.push(word),
            None => self.0 = Some(SExpr::List(vec![word])),
            Some(SExpr::Word(_) | SExpr::Keyword(_) | SExpr::Symbol(_) | SExpr::Constant(_)) => {
                unreachable!()
            }
        };

        Ok(self)
    }
}

impl ChildWriter for SExprBuilder<SExprBuilder<()>> {
    type Parent = SExpr;

    fn leave(self) -> Result<Self::Parent, Self::Error> {
        let Self(sexpr, mut parent) = self;
        let sexpr = sexpr.ok_or(Error::EmptyLeave)?;

        match parent.0 {
            Some(SExpr::Word(_) | SExpr::Constant(_) | SExpr::Symbol(_) | SExpr::Keyword(_)) => {
                unreachable!("parent has a word pending, shouldn't have been able to enter")
            }
            Some(SExpr::List(ref mut list)) => list.push(sexpr),
            None => parent.0 = Some(sexpr),
        }

        Ok(parent.0.unwrap())
    }
}
impl<T: Depth> ChildWriter for SExprBuilder<SExprBuilder<SExprBuilder<T>>> {
    type Parent = SExprBuilder<SExprBuilder<T>>;

    fn leave(self) -> Result<Self::Parent, Self::Error> {
        let Self(sexpr, mut parent) = self;
        let sexpr = sexpr.ok_or(Error::EmptyLeave)?;

        match parent.0 {
            Some(SExpr::Word(_) | SExpr::Constant(_) | SExpr::Symbol(_) | SExpr::Keyword(_)) => {
                unreachable!("parent has a word pending, shouldn't have been able to enter")
            }
            Some(SExpr::List(ref mut list)) => list.push(sexpr),
            None => parent.0 = Some(sexpr),
        }

        Ok(parent)
    }
}
