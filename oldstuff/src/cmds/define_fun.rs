use std::fmt::Debug;

use crate::{
    sexprs::{ChildWriter, Write, Writer},
    syntax::Symbol,
    FunctionDefinition, Term,
};

type FunctionDef<T> = FunctionDefinition<T>;

pub struct DefineFun<T: Term> {
    definiton: FunctionDefinition<T>,
    is_rec: bool,
}

impl<E: Term> Write for DefineFun<E> {
    type Error<W: crate::sexprs::Writer> = Error<W>;

    fn write<W: crate::sexprs::Writer>(&self, w: W) -> Result<W::Next, Self::Error<W>> {
        let cmd_symbol = if self.is_rec {
            Symbol::new("define-fun-rec".to_string())
        } else {
            Symbol::new("define-fun".to_string())
        };

        let inner = w
            .enter()
            .map_err(Error::WriteError)?
            .write_symbol(&cmd_symbol)
            .map_err(Error::WriteError)?;

        // let self.self.definiton.sig.name

        inner.leave().map_err(Error::WriteError)
    }
}

pub enum Error<W: Writer> {
    WriteError(W::Error),
}

impl<W: Writer> Debug for Error<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::WriteError(err) => f.debug_tuple("WriteError").field(err).finish(),
        }
    }
}

impl<W: Writer, E> crate::sexprs::WriteError<E> for Error<W> {}
