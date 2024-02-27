use crate::{Identifier, Sort, Term};

pub struct FunctionSignature {
    pub name: String,
    pub args: Vec<(Identifier, Sort)>,
    pub return_sort: Sort,
}

pub struct FunctionDefinition<E: Term> {
    pub sig: FunctionSignature,
    pub body: E,
}
