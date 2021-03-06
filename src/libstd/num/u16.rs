// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Operations and constants for unsigned 16-bits integers (`u16` type)

#[allow(non_uppercase_statics)];

use prelude::*;

use default::Default;
use from_str::FromStr;
use num::{Bitwise, Bounded};
use num::{CheckedAdd, CheckedSub, CheckedMul};
use num::{CheckedDiv, Zero, One, strconv};
use num::{ToStrRadix, FromStrRadix};
use option::{Option, Some, None};
use str;
use intrinsics;

uint_module!(u16, i16, 16)

impl CheckedAdd for u16 {
    #[inline]
    fn checked_add(&self, v: &u16) -> Option<u16> {
        unsafe {
            let (x, y) = intrinsics::u16_add_with_overflow(*self, *v);
            if y { None } else { Some(x) }
        }
    }
}

impl CheckedSub for u16 {
    #[inline]
    fn checked_sub(&self, v: &u16) -> Option<u16> {
        unsafe {
            let (x, y) = intrinsics::u16_sub_with_overflow(*self, *v);
            if y { None } else { Some(x) }
        }
    }
}

impl CheckedMul for u16 {
    #[inline]
    fn checked_mul(&self, v: &u16) -> Option<u16> {
        unsafe {
            let (x, y) = intrinsics::u16_mul_with_overflow(*self, *v);
            if y { None } else { Some(x) }
        }
    }
}
