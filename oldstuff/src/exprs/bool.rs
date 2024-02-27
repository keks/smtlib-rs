// pub use eq::{EqWriter, IterEq, WriteError as EqWriterError};
// pub use eq2::Eq2;
pub use literal::Literal;

mod literal {
    use crate::syntax::{lexicon::Symbol, sexprs::Sexpr};

    pub struct Literal(pub bool);

    impl From<Literal> for Symbol {
        fn from(value: Literal) -> Self {
            if value.0 {
                Symbol::new("true".to_string()).unwrap()
            } else {
                Symbol::new("false".to_string()).unwrap()
            }
        }
    }

    impl From<Literal> for Sexpr {
        fn from(value: Literal) -> Self {
            let sym: Symbol = value.into();
            sym.into()
        }
    }
}

// mod literal {
//     use std::convert::Infallible;
//
//     use crate::{
//         sexprs::{Write, Writer},
//         Sort, Term,
//     };
//
//     #[derive(Clone, Copy, Debug)]
//     pub struct Literal(pub bool);
//
//     impl Write for Literal {
//         type Error<W: Writer> = W::Error;
//         fn write<W: Writer>(&self, w: W) -> Result<W::Next, W::Error> {
//             w.write_word(&format!("{}", self.0))
//         }
//     }
//
//     impl Term for Literal {
//         type TypecheckError = Infallible;
//
//         fn sort(&self) -> Sort {
//             Sort::Bool
//         }
//
//         fn typecheck(&self) -> Result<(), Self::TypecheckError> {
//             Ok(())
//         }
//     }
//
//     impl crate::BlanketMarker for Literal {}
// }
//
// mod eq2 {
//     use crate::{
//         sexprs::{ChildWriter, Writer},
//         Sort, Term,
//     };
//
//     #[derive(Clone, Debug)]
//     pub struct Eq2<L, R>(pub L, pub R)
//     where
//         L: Term,
//         R: Term;
//
//     impl<L, R> crate::BlanketMarker for Eq2<L, R>
//     where
//         L: Term,
//         R: Term,
//     {
//     }
//
//     #[derive(Clone)]
//     pub enum WriteError<L: Term, R: Term, W: Writer> {
//         WriteError(W::Error),
//         WriteLeftExprError(L::Error<W::Child>),
//         WriteRightExprError(R::Error<W::Child>),
//     }
//
//     impl<L: Term, R: Term, W: Writer> std::fmt::Debug for WriteError<L, R, W> {
//         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//             match self {
//                 WriteError::WriteError(err) => {
//                     f.debug_tuple("Error::WriteError").field(err).finish()
//                 }
//                 WriteError::WriteLeftExprError(err) => {
//                     f.debug_tuple("WriteLeftExprError").field(err).finish()
//                 }
//                 WriteError::WriteRightExprError(err) => {
//                     f.debug_tuple("WriteRightExprError").field(err).finish()
//                 }
//             }
//         }
//     }
//
//     impl<L: Term, R: Term, W: Writer> WriteError<L, R, W> {
//         fn write_error(err: W::Error) -> Self {
//             Self::WriteError(err)
//         }
//         fn write_left_expr_error(err: L::Error<W::Child>) -> Self {
//             Self::WriteLeftExprError(err)
//         }
//         fn write_right_expr_error(err: R::Error<W::Child>) -> Self {
//             Self::WriteRightExprError(err)
//         }
//     }
//
//     impl<L: Term, R: Term, W: Writer> crate::sexprs::WriteError<W::Error> for WriteError<L, R, W> {}
//
//     impl<L: Term, R: Term> crate::sexprs::Write for Eq2<L, R> {
//         type Error<W: Writer> = WriteError<L, R, W>;
//
//         fn write<W: crate::sexprs::Writer>(&self, w: W) -> Result<W::Next, Self::Error<W>> {
//             let mut inner = w.enter().map_err(WriteError::write_error)?;
//             inner = inner.write_word("=").map_err(WriteError::write_error)?;
//             inner = self
//                 .0
//                 .write(inner)
//                 .map_err(WriteError::write_left_expr_error)?;
//             inner = self
//                 .1
//                 .write(inner)
//                 .map_err(WriteError::write_right_expr_error)?;
//             inner.leave().map_err(WriteError::write_error)
//         }
//     }
//
//     pub struct InputSortsDontMatch<L: Term, R: Term>(L, R);
//
//     impl<L: Term, R: Term> Term for Eq2<L, R> {
//         type TypecheckError = InputSortsDontMatch<L, R>;
//
//         fn sort(&self) -> Sort {
//             Sort::Bool
//         }
//
//         fn typecheck(&self) -> Result<(), Self::TypecheckError> {
//             if self.0.sort() != self.1.sort() {
//                 return Err(InputSortsDontMatch(self.0.clone(), self.1.clone()));
//             } else {
//                 Ok(())
//             }
//         }
//     }
// }
//
// mod eq {
//     use std::{convert::Infallible, marker::PhantomData};
//
//     use crate::{
//         sexprs::{ChildWriter, Write, WriteError as SExprWriteError, Writer},
//         Sort, Term,
//     };
//
//     enum EqWriterState<W: Writer> {
//         Init(W),
//         Inner(Sort, W::Child),
//     }
//
//     pub struct EqWriter<W: Writer>(EqWriterState<W>);
//
//     #[derive(Debug, Clone)]
//     pub enum WriteError<WE, EE> {
//         WriterError(WE),
//         WriteExprError(EE),
//         EmptyFinish,
//         SortMismatch(Sort, Sort),
//     }
//
//     impl<W: Writer> EqWriter<W> {
//         pub fn new(w: W) -> Self {
//             Self(EqWriterState::Init(w))
//         }
//
//         pub fn write<E: Term>(
//             self,
//             e: &E,
//         ) -> Result<Self, WriteError<W::Error, E::Error<W::Child>>> {
//             let inner = match self.0 {
//                 EqWriterState::Init(w) => w
//                     .enter()
//                     .map_err(WriteError::WriterError)?
//                     .write_word("=")
//                     .map_err(WriteError::WriterError)?,
//
//                 EqWriterState::Inner(sort, inner) => {
//                     if e.sort() != sort {
//                         return Err(WriteError::SortMismatch(sort, e.sort()));
//                     }
//                     inner
//                 }
//             };
//
//             let next_inner = e.write(inner).map_err(WriteError::WriteExprError)?;
//             Ok(EqWriter(EqWriterState::Inner(e.sort(), next_inner)))
//         }
//
//         pub fn finish(self) -> Result<W::Next, WriteError<W::Error, Infallible>> {
//             match self.0 {
//                 EqWriterState::Init(_) => Err(WriteError::EmptyFinish),
//                 EqWriterState::Inner(_, inner) => inner.leave().map_err(WriteError::WriterError),
//             }
//         }
//     }
//
//     pub struct IterEq<E: Term, R: AsRef<E>, I: Clone + Iterator<Item = R>>(
//         I,
//         PhantomData<(E, R, I)>,
//     );
//
//     impl<E: Term, R: AsRef<E>, I: Clone + Iterator<Item = R>> IterEq<E, R, I> {
//         pub fn new(iter: I) -> Self {
//             Self(iter, PhantomData)
//         }
//     }
//
//     impl<WE, EE> SExprWriteError<WE> for WriteError<WE, EE>
//     where
//         WE: crate::sexprs::WriteError<WE>,
//         EE: crate::sexprs::WriteError<WE>,
//     {
//     }
//
//     impl<E: Term, R: AsRef<E>, I: Clone + Iterator<Item = R>> Write for IterEq<E, R, I> {
//         type Error<W: Writer> = WriteError<W::Error, E::Error<W::Child>>;
//
//         fn write<W: Writer>(&self, w: W) -> Result<W::Next, Self::Error<W>> {
//             let mut eq_writer = EqWriter::new(w);
//             for expr in self.0.clone() {
//                 eq_writer = eq_writer.write(expr.as_ref())?;
//             }
//             eq_writer.finish().map_err(|e| match e {
//                 WriteError::WriterError(err) => WriteError::WriterError(err),
//                 WriteError::WriteExprError(infallible) => match infallible {},
//                 WriteError::EmptyFinish => WriteError::EmptyFinish,
//                 WriteError::SortMismatch(s1, s2) => WriteError::SortMismatch(s1, s2),
//             })
//         }
//     }
// }
