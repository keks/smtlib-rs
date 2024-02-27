use crate::{
    sexprs::{ChildWriter, Write, WriteError, Writer},
    sorts::SortError,
    BlanketMarker, Sort, Term,
};

pub struct Assert<E: Term>(pub E);

pub enum Error<W: Writer, E: Term> {
    NotBoolExpr(SortError<E>),
    WriteError(W::Error),
    WriteExprError(E::Error<W::Child>),
}

impl<W: Writer, E: Term> std::fmt::Debug for Error<W, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotBoolExpr(err) => f
                .debug_tuple(&format!("Error::NotBoolExpr"))
                .field(err)
                .finish(),
            Error::WriteError(err) => f
                .debug_tuple(&format!("Error::WriteError"))
                .field(err)
                .finish(),
            Error::WriteExprError(err) => f
                .debug_tuple(&format!("Error::WriteExprError"))
                .field(err)
                .finish(),
        }
    }
}

impl<W: Writer, E: Term> Error<W, E> {
    fn not_bool_expr(expr: E) -> Self {
        Self::NotBoolExpr(SortError::new(expr, Sort::Bool))
    }

    fn write_error(err: W::Error) -> Self {
        Self::WriteError(err)
    }

    fn write_expr_error(err: E::Error<W::Child>) -> Self {
        Self::WriteExprError(err)
    }
}

impl<W: Writer, E: Term> WriteError<W::Error> for Error<W, E> {}

impl<E: Term> BlanketMarker for Assert<E> {}

impl<E: Term> Write for Assert<E> {
    type Error<W: Writer> = Error<W, E>;

    fn write<W: Writer>(&self, w: W) -> Result<W::Next, Error<W, E>> {
        if !matches!(self.0.sort(), Sort::Bool) {
            return Err(Error::not_bool_expr(self.0.clone()));
        }

        let mut inner = w.enter().map_err(Error::write_error)?;
        inner = inner.write_word("assert").map_err(Error::write_error)?;
        inner = self.0.write(inner).map_err(Error::write_expr_error)?;
        inner.leave().map_err(Error::write_error)
    }
}
