pub struct Boo {}

pub trait BooShape {
    fn new() -> Self;
}

impl BooShape for Boo {
    fn new() -> Self {
        return Boo {};
    }
}
