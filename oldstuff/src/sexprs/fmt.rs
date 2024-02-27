use super::{ChildWriter, WriteError, Writer};

trait Nat {}
pub struct Zero;
#[allow(private_bounds)]
pub struct Succ<N: Nat>(N);
impl Nat for Zero {}
impl<N: Nat> Nat for Succ<N> {}

pub struct FmtWriter<W: std::fmt::Write, N>(W, N, bool);

impl<W: std::fmt::Write> FmtWriter<W, Zero> {
    pub fn new(w: W) -> Self {
        Self(w, Zero, true)
    }

    pub fn finish(self) -> W {
        self.0
    }
}

impl WriteError<Self> for std::fmt::Error {}

impl<W: std::fmt::Write> Writer for FmtWriter<W, Zero> {
    type Error = std::fmt::Error;
    type Child = FmtWriter<W, Succ<Zero>>;
    type Next = W;

    fn enter(mut self) -> Result<Self::Child, Self::Error> {
        let FmtWriter(ref mut w, _, new) = self;
        if new {
            write!(w, "(")?;
        } else {
            write!(w, " (")?;
        }
        Ok(FmtWriter(self.0, Succ(self.1), true))
    }

    fn write_word(mut self, word: &str) -> Result<Self::Next, Self::Error> {
        write!(&mut self.0, "{word}")?;
        Ok(self.0)
    }
}

impl<W: std::fmt::Write, N: Nat> Writer for FmtWriter<W, Succ<N>> {
    type Error = std::fmt::Error;
    type Child = FmtWriter<W, Succ<Succ<N>>>;
    type Next = Self;

    fn write_word(mut self, word: &str) -> Result<Self::Next, Self::Error> {
        let FmtWriter(ref mut w, _, new) = self;

        if new {
            write!(w, "{word}")?;
        } else {
            write!(w, " {word}")?;
        }

        self.2 = false;
        Ok(self)
    }
    fn enter(mut self) -> Result<Self::Child, Self::Error> {
        let FmtWriter(ref mut w, _, new) = self;
        if new {
            write!(w, "(")?;
        } else {
            write!(w, " (")?;
        }
        Ok(FmtWriter(self.0, Succ(self.1), true))
    }
}

impl<W: std::fmt::Write> ChildWriter for FmtWriter<W, Succ<Zero>> {
    type Parent = W;

    fn leave(mut self) -> Result<Self::Parent, Self::Error> {
        write!(&mut self.0, ")")?;
        Ok(self.0)
    }
}
impl<W: std::fmt::Write, N: Nat> ChildWriter for FmtWriter<W, Succ<Succ<N>>> {
    type Parent = FmtWriter<W, Succ<N>>;

    fn leave(mut self) -> Result<Self::Parent, Self::Error> {
        write!(&mut self.0, ")")?;
        Ok(FmtWriter(self.0, self.1 .0, false))
    }
}
