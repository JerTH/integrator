

pub trait Coincident<Rhs = Self> {
    fn coincident(&self, other: Rhs) -> bool;
}

pub trait Parallel<Rhs = Self> {
    fn parallel(self, other: Rhs) -> bool;
}

