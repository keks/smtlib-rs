use crate::{syntax::Symbol, Sort};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selector {
    pub name: Symbol,
    pub sort: Sort,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Constructor {
    pub name: Symbol,
    pub selectors: Vec<Selector>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Datatype {
    pub name: Symbol,
    pub constructors: Vec<Constructor>,
}
