//
//  SOS: the Stupid Operating System
//  by Eliza Weisman (eliza@elizas.website)
//
//  Copyright (c) 2016 Eliza Weisman
//  Released under the terms of the MIT license. See `LICENSE` in the root
//  directory of this repository for more information.
//
//! General purpose stuff I couldn't find a better home for.
#![crate_name = "util"]
#![no_std]

#![feature(step_trait)]
// #[cfg(not(test))] extern crate vga;

use core::{fmt, ops};
use ops::*;
use core::iter::Step;
// use core::num::One;

pub mod io;

#[macro_use] pub mod macros;

/// The unreachable Void type.
pub enum Void {}
impl fmt::Debug for Void {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }
}

pub trait Align: Sized + Copy //+ One
               + Add<Output=Self> + Sub<Output=Self>
               + BitAnd<Output=Self> + Not<Output=Self>
               + Step
{
    #[inline] fn align_up(&self, to: Self) -> Self {
        let align = to.sub_one();
        (*self + align) & !align
    }
    #[inline] fn align_down(&self, to: Self) -> Self {
        *self & !to.sub_one()
    }
}

impl Align for u64 { }
impl Align for u32 { }
impl Align for usize { }
