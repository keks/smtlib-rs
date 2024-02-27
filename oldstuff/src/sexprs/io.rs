use super::{ChildWriter, WriteError, Writer};

trait Nat {}
pub struct Zero;
#[allow(private_bounds)]
pub struct Succ<N: Nat>(N);
impl Nat for Zero {}
impl<N: Nat> Nat for Succ<N> {}

pub struct IOWriter<W: std::io::Write, N>(W, N, bool);

impl<W: std::io::Write> IOWriter<W, Zero> {
    pub fn new(w: W) -> Self {
        Self(w, Zero, true)
    }

    pub fn finish(self) -> W {
        self.0
    }
}

impl WriteError<Self> for std::io::Error {}

impl<W: std::io::Write> Writer for IOWriter<W, Zero> {
    type Error = std::io::Error;
    type Child = IOWriter<W, Succ<Zero>>;
    type Next = W;

    fn enter(mut self) -> Result<Self::Child, Self::Error> {
        let IOWriter(ref mut w, _, new) = self;
        if new {
            write!(w, "(")?;
        } else {
            write!(w, " (")?;
        }
        Ok(IOWriter(self.0, Succ(self.1), true))
    }

    fn write_word(mut self, word: &str) -> Result<Self::Next, Self::Error> {
        write!(&mut self.0, "{word}")?;
        Ok(self.0)
    }
}

impl<W: std::io::Write, N: Nat> Writer for IOWriter<W, Succ<N>> {
    type Error = std::io::Error;
    type Child = IOWriter<W, Succ<Succ<N>>>;
    type Next = Self;

    fn write_word(mut self, word: &str) -> Result<Self::Next, Self::Error> {
        let IOWriter(ref mut w, _, new) = self;

        if new {
            write!(w, "{word}")?;
        } else {
            write!(w, " {word}")?;
        }

        self.2 = false;
        Ok(self)
    }
    fn enter(mut self) -> Result<Self::Child, Self::Error> {
        let IOWriter(ref mut w, _, new) = self;
        if new {
            write!(w, "(")?;
        } else {
            write!(w, " (")?;
        }
        Ok(IOWriter(self.0, Succ(self.1), true))
    }
}

impl<W: std::io::Write> ChildWriter for IOWriter<W, Succ<Zero>> {
    type Parent = W;

    fn leave(mut self) -> Result<Self::Parent, Self::Error> {
        write!(&mut self.0, ")")?;
        Ok(self.0)
    }
}
impl<W: std::io::Write, N: Nat> ChildWriter for IOWriter<W, Succ<Succ<N>>> {
    type Parent = IOWriter<W, Succ<N>>;

    fn leave(mut self) -> Result<Self::Parent, Self::Error> {
        write!(&mut self.0, ")")?;
        Ok(IOWriter(self.0, self.1 .0, false))
    }
}
