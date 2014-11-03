// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(exceeding_bitshifts)]
#![allow(unused_variables)]

fn main() {
      let n = 1u8 << 8;
      let n = 1u8 << 9;   //~ ERROR: bitshift exceeds the type's number of bits
      let n = 1u16 << 16;
      let n = 1u16 << 17; //~ ERROR: bitshift exceeds the type's number of bits
      let n = 1u32 << 32;
      let n = 1u32 << 33; //~ ERROR: bitshift exceeds the type's number of bits
      let n = 1u64 << 64;
      let n = 1u64 << 65; //~ ERROR: bitshift exceeds the type's number of bits
      let n = 1i8 << 8;
      let n = 1i8 << 9;   //~ ERROR: bitshift exceeds the type's number of bits
      let n = 1i16 << 16;
      let n = 1i16 << 17; //~ ERROR: bitshift exceeds the type's number of bits
      let n = 1i32 << 32;
      let n = 1i32 << 33; //~ ERROR: bitshift exceeds the type's number of bits
      let n = 1i64 << 64;
      let n = 1i64 << 65; //~ ERROR: bitshift exceeds the type's number of bits

      let n = 1u8 >> 8;
      let n = 1u8 >> 9;   //~ ERROR: bitshift exceeds the type's number of bits
      let n = 1u16 >> 16;
      let n = 1u16 >> 17; //~ ERROR: bitshift exceeds the type's number of bits
      let n = 1u32 >> 32;
      let n = 1u32 >> 33; //~ ERROR: bitshift exceeds the type's number of bits
      let n = 1u64 >> 64;
      let n = 1u64 >> 65; //~ ERROR: bitshift exceeds the type's number of bits
      let n = 1i8 >> 8;
      let n = 1i8 >> 9;   //~ ERROR: bitshift exceeds the type's number of bits
      let n = 1i16 >> 16;
      let n = 1i16 >> 17; //~ ERROR: bitshift exceeds the type's number of bits
      let n = 1i32 >> 32;
      let n = 1i32 >> 33; //~ ERROR: bitshift exceeds the type's number of bits
      let n = 1i64 >> 64;
      let n = 1i64 >> 65; //~ ERROR: bitshift exceeds the type's number of bits

      let n = 1u8;
      let n = n << 8;
      let n = n << 9; //~ ERROR: bitshift exceeds the type's number of bits

      let n = 1u8 << -9; //~ ERROR: bitshift exceeds the type's number of bits

      let n = 1u8 << (4+4);
      let n = 1u8 << (4+5); //~ ERROR: bitshift exceeds the type's number of bits
}
