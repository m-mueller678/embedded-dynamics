extern crate core;

use std::fs;
use std::fs::File;
use goblin::elf::program_header::PT_LOAD;
use goblin::elf::Elf;
use std::io::Write;

fn main()->std::io::Result<()>{
    let elf_path="../guest/target/thumbv6m-none-eabi/release/guest";
    println!("cargo:rerun-if-changed={}",elf_path);
    let buffer = fs::read(elf_path).unwrap();
    let elf=Elf::parse(&buffer).unwrap();
    let mut out_file = File::create("src/generated_guest.rs")?;
    let phs=elf.program_headers.iter().filter(|ph|{
        ph.p_type == PT_LOAD
    }).map(|ph|{
        let mut data=buffer[ph.p_offset as usize..][..ph.p_filesz as usize].to_vec();
        data.resize(ph.p_memsz as usize,0);
        (ph.p_vaddr, data)
    }).collect::<Vec<_>>();


    writeln!(out_file,"pub const PROGRAMM_HEADERS:&[(u32,&[u8])] = &[")?;
    for ph in phs{
        writeln!(out_file,"\t(0x{:x},&{:?}),",ph.0,ph.1)?;
    }
    writeln!(out_file,"];\n")?;
    writeln!(out_file,"pub const ENTRY_POINT:u32 = 0x{:x};",elf.entry)?;
    Ok(())
}