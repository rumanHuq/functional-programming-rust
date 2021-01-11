mod whatever;
use std::{fmt::Debug, ops::Mul};
use whatever::{Boo, BooShape};

fn closure<F, T>(function: F, num: T) -> T
where
    F: Fn(T) -> T,
    T: Mul<Output = T> + Debug,
{
    return function(num);
}

fn main() {
    let val = closure(|num| num + num, 0.1);
    println!("{}", val);

    let boo = Boo::new();
}
