use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
struct One<T: Display> {
    inner: OneOne<T>,
}

#[derive(Debug)]
struct OneOne<T: Display> {
    v: T,
}

impl<T: Display> Deref for One<T> {
    type Target = OneOne<T>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Display> DerefMut for One<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

fn main() {
    let one = One {
        inner: OneOne { v: 1 },
    };
    print_one_one(&one);

    let mut mut_one = One {
        inner: OneOne { v: 2 },
    };
    print_one_one(&mut mut_one);

    update_one_one(&mut mut_one, 3);

    print_one_one(&mut mut_one);
}

fn print_one_one<T: Display>(one_one: &OneOne<T>) {
    println!("{}", one_one.v);
}

fn update_one_one<T: Display>(one_one: &mut OneOne<T>, u: T) {
    one_one.v = u
}
