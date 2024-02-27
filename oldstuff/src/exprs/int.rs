pub use add2::Add2;
pub use literal::Literal;

mod literal {
    use std::convert::Infallible;

    use crate::{
        sexprs::{Write, Writer},
        Sort, Term,
    };

    #[derive(Clone, Copy, Debug)]
    pub struct Literal(pub isize);
    impl Write for Literal {
        fn write<W: Writer>(&self, w: W) -> Result<W::Next, W::Error> {
            w.write_word(&format!("{}", self.0))
        }

        type Error<W: Writer> = W::Error;
    }

    impl Term for Literal {
        type TypecheckError = Infallible;
        fn sort(&self) -> Sort {
            Sort::Int
        }
        fn typecheck(&self) -> Result<(), Self::TypecheckError> {
            Ok(())
        }
    }
    impl crate::BlanketMarker for Literal {}

    impl From<isize> for Literal {
        fn from(value: isize) -> Self {
            Self(value)
        }
    }
}

mod add2 {
    use crate::{
        sexprs::{ChildWriter, Write, Writer},
        Sort, Term,
    };

    #[derive(Clone, Copy, Debug)]
    pub struct Add2<L, R>(pub L, pub R)
    where
        L: Term,
        R: Term;

    #[derive(Debug)]
    pub struct NotAddableError<L: Term, R: Term>(L, R);

    impl<L, R> Term for Add2<L, R>
    where
        L: Term,
        R: Term,
    {
        type TypecheckError = NotAddableError<L, R>;
        fn sort(&self) -> Sort {
            Sort::Int
        }

        fn typecheck(&self) -> Result<(), Self::TypecheckError> {
            if self.0.sort() != Sort::Int || self.1.sort() != Sort::Int {
                Err(NotAddableError(self.0.clone(), self.1.clone()))
            } else {
                Ok(())
            }
        }
    }

    pub enum WriteError<L: Term, R: Term, W: Writer> {
        WriteError(W::Error),
        WriteLeftTermError(L::Error<W::Child>),
        WriteRightTermError(R::Error<W::Child>),
    }

    impl<L: Term, R: Term, W: Writer> crate::sexprs::WriteError<W::Error> for WriteError<L, R, W> {}

    impl<L: Term, R: Term, W: Writer> std::fmt::Debug for WriteError<L, R, W> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                WriteError::WriteError(err) => {
                    f.debug_tuple("Error::WriteError").field(err).finish()
                }
                WriteError::WriteLeftTermError(err) => {
                    f.debug_tuple("WriteLeftTermError").field(err).finish()
                }
                WriteError::WriteRightTermError(err) => {
                    f.debug_tuple("WriteRightTermError").field(err).finish()
                }
            }
        }
    }

    impl<L: Term, R: Term, W: Writer> WriteError<L, R, W> {
        fn write_error(err: W::Error) -> Self {
            Self::WriteError(err)
        }
        fn write_left_expr_error(err: L::Error<W::Child>) -> Self {
            Self::WriteLeftTermError(err)
        }
        fn write_right_expr_error(err: R::Error<W::Child>) -> Self {
            Self::WriteRightTermError(err)
        }
    }

    impl<L, R> Write for Add2<L, R>
    where
        L: Term,
        R: Term,
    {
        type Error<W: Writer> = WriteError<L, R, W>;

        fn write<W: Writer>(&self, w: W) -> Result<W::Next, WriteError<L, R, W>> {
            let mut inner = w.enter().map_err(WriteError::write_error)?;
            inner = inner.write_word("+").map_err(WriteError::write_error)?;
            inner = self
                .0
                .write(inner)
                .map_err(WriteError::write_left_expr_error)?;
            inner = self
                .1
                .write(inner)
                .map_err(WriteError::write_right_expr_error)?;
            inner.leave().map_err(WriteError::write_error)
        }
    }

    impl<L, R> crate::BlanketMarker for Add2<L, R>
    where
        L: Term,
        R: Term,
    {
    }
}

macro_rules! impl_Add {
    ( < $($type_params:ident),+ > $name:ty) => {
        impl<$($type_params),+ , T> ::core::ops::Add<T> for $name
        where
            T: $crate::Term,
            $($type_params: $crate::Term),+
        {
            type Output = $crate::exprs::int::Add2<$name, T>;

            fn add(self, rhs: T) -> Self::Output {
                $crate::exprs::int::Add2(self, rhs)
            }
        }
    };
    ($name:ty) => {
        impl<T: $crate::Term> ::core::ops::Add<T>
            for $name
        {
            type Output = $crate::exprs::int::Add2<$name, T>;

            fn add(self, rhs: T) -> Self::Output {
                $crate::exprs::int::Add2(self, rhs)
            }
        }
    };
}

impl_Add!(Literal);
impl_Add!(<U, V> Add2<U, V>);
