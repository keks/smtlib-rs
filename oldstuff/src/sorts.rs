use crate::{
    sexprs::{Write, Writer},
    Sort, Term,
};

impl Write for Sort {
    type Error<W: Writer> = W::Error;

    fn write<W: Writer>(&self, w: W) -> Result<W::Next, W::Error> {
        match self {
            Sort::Int => w.write_word("Int"),
            Sort::Bool => w.write_word("Bool"),
            Sort::Datatype(datatype) => w.write_symbol(&datatype.name),
        }
    }
}

#[derive(Debug)]
pub struct SortError<E: Term> {
    expr: E,
    expect_sort: Sort,
}

impl<E: Term> SortError<E> {
    pub fn new(expr: E, expect_sort: Sort) -> Self {
        Self { expr, expect_sort }
    }

    pub fn expr(&self) -> &E {
        &self.expr
    }

    pub fn expect_sort(&self) -> &Sort {
        &self.expect_sort
    }
}
