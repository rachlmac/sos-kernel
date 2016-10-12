//
//  SOS: the Stupid Operating System
//  by Hawk Weisman (hi@hawkweisman.me)
//
//  Copyright (c) 2015 Hawk Weisman
//  Released under the terms of the MIT license. See `LICENSE` in the root
//  directory of this repository for more information.
//
//! The Global Descriptor Table (GDT) is used for configuring segmentation.
//!
//! As we use paging rather than segmentation for memory management, we do
//! not actually use the GDT, but some x86 functionality still require itg
//! to be properly configured.
use arch::cpu::segment::*;

const GDT_SIZE: usize = 3;

type Gdt = [Descriptor; GDT_SIZE];

static GDT64: Gdt
    = [ Descriptor::null()
        // code segment
      , Descriptor { base_high: 0b0000_0000
                   , flags: Flags::from_raw(
                        (1<<4) | (1<<7) | (1<<1) | (1<<3) | (1<<13) )
                   , base_mid: 0
                   , base_low: 0
                   , limit: 0 }
        // data segment
      , Descriptor { base_high: 0b0000_0000
                   , flags: Flags::from_raw( (1<<4) | (1<<7) | (1<<1) )
                   , base_mid: 0
                   , base_low: 0
                   , limit: 0 }
      ];
