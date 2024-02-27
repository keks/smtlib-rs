use crate::{
    sexprs::{ChildWriter, Write, Writer},
    BlanketMarker, Identifier, Sort,
};

pub struct DeclareConst {
    pub ident: Identifier,
    pub sort: Sort,
}

impl BlanketMarker for DeclareConst {}

impl Write for DeclareConst {
    type Error<W: Writer> = W::Error;

    fn write<W: Writer>(&self, w: W) -> Result<W::Next, W::Error> {
        let mut inner = w.enter()?;
        inner = inner.write_word("declare-const")?;
        inner = self.ident.write(inner)?;
        inner = self.sort.write(inner)?;
        inner.leave()
    }
}
