// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Forbid assignment into a dynamically sized type.

#![feature(unsized_tuple_coercion)]

type Fat<T: ?Sized> = (isize, &'static str, T);
//~^ WARNING trait bounds are not (yet) enforced

#[derive(PartialEq,Eq)]
struct Bar;

#[derive(PartialEq,Eq)]
struct Bar1 {
    f: isize
}

trait ToBar {
    fn to_bar(&self) -> Bar;
    fn to_val(&self) -> isize;
}

impl ToBar for Bar1 {
    fn to_bar(&self) -> Bar {
        Bar
    }
    fn to_val(&self) -> isize {
        self.f
    }
}

pub fn main() {
    // Assignment.
    let f5: &mut Fat<ToBar> = &mut (5, "some str", Bar1 {f :42});
    let z: Box<ToBar> = Box::new(Bar1 {f: 36});
    f5.2 = Bar1 {f: 36};
    //~^ ERROR mismatched types
    //~| expected type `ToBar`
    //~| found type `Bar1`
    //~| expected trait ToBar, found struct `Bar1`
    //~| ERROR `ToBar: std::marker::Sized` is not satisfied
}
