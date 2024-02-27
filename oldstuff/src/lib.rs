// pub mod cmds;
pub mod exprs;
// pub mod sexprs;
// pub mod sorts;

// pub mod build;
pub mod syntax;
// pub mod write;

// mod datatypes;
// mod functions;

// pub use datatypes::{Constructor, Datatype, Selector};
// pub use functions::{FunctionDefinition, FunctionSignature};

// use sexprs::{ChildWriter, SExpr, ToSExpr, Write, Writer};

// this is a private market trait used for blanket impls
// pub(crate) trait BlanketMarker {}

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub enum Sort {
//     Int,
//     Bool,
//     Datatype(Datatype),
// }
//
// pub trait Command: sexprs::Write {}
//
// pub trait Term: sexprs::Write + std::fmt::Debug + Clone {
//     type TypecheckError;
//
//     fn sort(&self) -> Sort;
//     fn typecheck(&self) -> Result<(), Self::TypecheckError>;
// }

#[cfg(test)]
mod tests {
    // use crate::cmds::{Assert, DeclareDatatype};
    // use crate::exprs::bool::Eq2;
    // use crate::exprs::int::Literal;
    // use crate::sexprs::{self, ToSExpr, Write};
    // use crate::syntax::Symbol;
    // use crate::{Constructor, Datatype, Selector, Sort};

    #[test]
    fn foo() {
        // let a: Literal = 9.into();
        // let b: Literal = 16.into();
        // let c: Literal = 25.into();
        //
        // // check that it implements the trait
        // let _sexpr = a.to_sexpr();
        // let _sexpr = (a + b).to_sexpr();
        //
        // let command = Assert(Eq2(a + b, c));
        //
        // let builder = sexprs::builder::SExprBuilder::new();
        // let sexpr = command.write(builder).unwrap();
        // println!("{sexpr}");
        //
        // let writer = sexprs::io::IOWriter::new(vec![]);
        // let bytes = command.write(writer).unwrap();
        // let string = String::from_utf8(bytes).unwrap();
        // println!("{}", string);
        //
        // let dt = DeclareDatatype(Datatype {
        //     name: Symbol::new("NewSort".to_string()),
        //     constructors: vec![Constructor {
        //         name: Symbol::new("mkNewSort".to_string()),
        //         selectors: vec![Selector {
        //             name: Symbol::new("NewSort-i".to_string()),
        //             sort: Sort::Int,
        //         }],
        //     }],
        // });
        //
        // let writer = sexprs::io::IOWriter::new(vec![]);
        // let bytes = dt.write(writer).unwrap();
        // let string = String::from_utf8(bytes).unwrap();
        // println!("{}", string);
    }
}
