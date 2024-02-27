use std::fmt::Debug;

use crate::{
    sexprs::{ChildWriter, Write, WriteError, Writer},
    Constructor, Datatype, Selector,
};

pub struct DeclareDatatype(pub Datatype);

/* Grammar:
 *
 * (declare-datatype <symbol> <datatype_dec>)  -- #1
 * <datatype_dec> ::= ( <construtor_dec>+ ) | (par ( <symbol>+ ) ( <constructor_dec>+ ) ) -- #2
 * <constructor_dec> ::= (<symbol> <selector_dec>+ ) -- #3
 * <selector_dec> ::= ( <symbol> <sort> ) -- #4
 */
impl Write for DeclareDatatype {
    type Error<W: Writer> = Error<W>;

    fn write<W: Writer>(&self, w: W) -> Result<W::Next, Self::Error<W>> {
        let datatype_dec = w
            .enter() // -- #1
            .map_err(Error::WriterError)?
            .write_word("declare-datatype")
            .map_err(Error::WriterError)?
            .write_symbol(&self.0.name)
            .map_err(Error::WriterError)?
            .enter() // -- #2
            .map_err(Error::WriterError)?;

        self.0
            .constructors
            .iter()
            .fold(Ok(datatype_dec), |w, constructor| {
                constructors_foldfn(constructor, w)
            })
            .map_err(|e| match e {
                Error::WriterError(e) => Error::WriterError(e),
                Error::WriteSortError(e) => Error::WriteSortError(e),
            })?
            .leave() // -- #2
            .map_err(Error::WriterError)?
            .leave() // -- #1
            .map_err(Error::WriterError)
    }
}

// <constructor_dec> ::= (<symbol> <selector_dec>+ ) -- #3
fn constructors_foldfn<W: Writer<Next = W>>(
    constructor: &Constructor,
    w: Result<W, Error<W>>,
) -> Result<W, Error<W>> {
    let selector_dec: W::Child = w?
        .enter() // -- #3
        .map_err(Error::WriterError)?
        .write_symbol(&constructor.name)
        .map_err(Error::WriterError)?;

    constructor
        .selectors
        .iter()
        .fold(Ok(selector_dec), |w, sel| selector_foldfn(sel, w))
        .map_err(|e| match e {
            Error::WriterError(e) => Error::WriterError(e),
            Error::WriteSortError(e) => Error::WriteSortError(e),
        })?
        .leave() // -- #3
        .map_err(Error::WriterError)
}

// <selector_dec> ::= ( <symbol> <sort> ) -- #4
fn selector_foldfn<W: Writer<Next = W>>(
    selector: &Selector,
    w: Result<W, Error<W>>,
) -> Result<W, Error<W>> {
    let mut inner = w?.enter().map_err(Error::WriterError)?;
    inner = inner
        .write_symbol(&selector.name)
        .map_err(Error::WriterError)?;
    inner = selector.sort.write(inner).map_err(Error::WriteSortError)?;
    inner.leave().map_err(Error::WriterError)
}

pub enum Error<W: Writer> {
    WriterError(W::Error),
    WriteSortError(W::Error),
}

impl<W: Writer> Debug for Error<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::WriterError(err) => f.debug_tuple("WriterError").field(err).finish(),
            Error::WriteSortError(err) => f.debug_tuple("WriteSortError").field(err).finish(),
        }
    }
}

impl<W: Writer> WriteError<W::Error> for Error<W> {}
