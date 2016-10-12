//
//  SOS: the Stupid Operating System
//  by Hawk Weisman (hi@hawkweisman.me)
//
//  Copyright (c) 2015 Hawk Weisman
//  Released under the terms of the MIT license. See `LICENSE` in the root
//  directory of this repository for more information.
//
//! `x86_64` architecture-specific implementation.
pub mod cpu;
pub mod drivers;
pub mod memory;
pub mod boot;

#[path = "../x86_all/bda.rs"] pub mod bda;
#[path = "../x86_all/multiboot2.rs"] pub mod multiboot2;

use memory::PAddr;

pub const ARCH_BITS: u8 = 64;

/// Entry point for architecture-specific kernel init
pub fn arch_init(multiboot_addr: PAddr) {

    // -- Unpack multiboot tag ------------------------------------------------
    let boot_info
        = unsafe { multiboot2::Info::from(multiboot_addr)
                    .expect("Could not unpack multiboot2 information!") };

    let mmap_tag // Extract the memory map tag from the multiboot info
        = boot_info.mem_map()
                   .expect("Memory map tag required!");

    println!(" . Detected memory areas:");
    for a in mmap_tag.areas() {
        println!(" . . start: {:#08x}, end: {:#08x}"
                , a.base, a.length );
    }

    let elf_sections_tag // Extract ELF sections tag from the multiboot info
        = boot_info.elf_sections()
                   .expect("ELF sections tag required!");

    println!(" . Detecting kernel ELF sections:");

    let kernel_begin    // Extract kernel ELF sections from  multiboot info
        = elf_sections_tag.sections()
            .map(|s| {
                println!(" . . address: {:#08x}, size: {:#08x}, flags: {:#08x}"
                        , s.addr()
                        , s.length()
                        , s.flags() );
                s.addr() })
            .min()
            .expect("Could not find kernel start section!\
                    \nSomething is deeply wrong.");

    let mut n_elf_sections = 0;
    let kernel_end
        = elf_sections_tag.sections()
            .map(|s| { n_elf_sections += 1; s.addr() })
            .max()
            .expect("Could not find kernel end section!\
                    \nSomething is deeply wrong.");

    println!( " . Detected {} kernel ELF sections.", n_elf_sections);
    println!( " . . Kernel begins at {:#x} and ends at {:#x}."
             , kernel_begin, kernel_end );

    let multiboot_end = multiboot_addr + boot_info.length as u64;

    println!( " . . Multiboot info begins at {:#x} and ends at {:#x}."
             , multiboot_addr, multiboot_end);

}
