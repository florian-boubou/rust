// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Tests that we can compare various kinds of extern fn signatures.


extern fn voidret1() {}
extern fn voidret2() {}

extern fn uintret() -> usize { 22 }

extern fn uintvoidret(_x: usize) {}

extern fn uintuintuintuintret(x: usize, y: usize, z: usize) -> usize { x+y+z }
type uintuintuintuintret = extern fn(usize,usize,usize) -> usize;

pub fn main() {
    assert!(voidret1 as extern fn() == voidret1 as extern fn());
    assert!(voidret1 as extern fn() != voidret2 as extern fn());

    assert!(uintret as extern fn() -> usize == uintret as extern fn() -> usize);

    assert!(uintvoidret as extern fn(usize) == uintvoidret as extern fn(usize));

    assert!(uintuintuintuintret as uintuintuintuintret ==
            uintuintuintuintret as uintuintuintuintret);
}
