use std::fmt::Display;

use super::identifiers::Identifier;
use super::lexicon::{Reserved, Symbol};
use super::sexprs::{Sexpr, SpecialConstant};
use super::sorts::Sort;

#[derive(Debug, Clone)]
pub struct QualifiedIdentifier(Identifier, Option<Sort>);
impl From<QualifiedIdentifier> for super::sexprs::Sexpr {
    fn from(value: QualifiedIdentifier) -> Self {
        let QualifiedIdentifier(id, sort) = value;

        match sort {
            Some(sort) => {
                Sexpr::Sequence(vec![Sexpr::Reserved(Reserved::As), id.into(), sort.into()])
            }
            None => id.into(),
        }
    }
}
impl Display for QualifiedIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let QualifiedIdentifier(id, sort) = self;

        match sort {
            Some(sort) => write!(f, "(as {id} {sort})"),
            None => write!(f, "{id}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VarBinding(Symbol, Term);

impl From<VarBinding> for Sexpr {
    fn from(value: VarBinding) -> Self {
        let VarBinding(name, term) = value;

        Sexpr::Sequence(vec![name.into(), term.into()])
    }
}

#[derive(Debug, Clone)]
pub struct SortedVar(Symbol, Sort);

impl From<SortedVar> for Sexpr {
    fn from(value: SortedVar) -> Self {
        let SortedVar(name, sort) = value;

        Sexpr::Sequence(vec![name.into(), sort.into()])
    }
}

#[derive(Debug, Clone)]
pub enum Term {
    SpecialConstant(SpecialConstant),
    QualifiedIdentifier(QualifiedIdentifier),
    Application(QualifiedIdentifier, Vec<Term>),
    Let(Vec<VarBinding>, Box<Term>),
    Forall(Vec<SortedVar>, Box<Term>),
    Exists(Vec<SortedVar>, Box<Term>),
    // TODO:
    // - match
    // - ! (term attributes)
}

impl From<Term> for Sexpr {
    fn from(value: Term) -> Self {
        match value {
            Term::SpecialConstant(sc) => sc.into(),
            Term::QualifiedIdentifier(qi) => qi.into(),
            Term::Application(name, args) => Sexpr::Sequence({
                vec![name.into()]
                    .into_iter()
                    .chain(args.into_iter().map(|e| e.into()))
                    .collect()
            }),
            Term::Let(bindings, body) => Sexpr::Sequence(vec![
                Reserved::Let.into(),
                Sexpr::Sequence(bindings.into_iter().map(|e| e.into()).collect()),
                (*body).into(),
            ]),
            Term::Forall(quants, body) => Sexpr::Sequence(vec![
                Reserved::Forall.into(),
                Sexpr::Sequence(quants.into_iter().map(|e| e.into()).collect()),
                (*body).into(),
            ]),
            Term::Exists(quants, body) => Sexpr::Sequence(vec![
                Reserved::Exists.into(),
                Sexpr::Sequence(quants.into_iter().map(|e| e.into()).collect()),
                (*body).into(),
            ]),
        }
    }
}
