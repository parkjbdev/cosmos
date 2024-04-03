use core::mem::{self, MaybeUninit};
use core::ops::Range;

use goblin::elf::section_header::section_header32::SectionHeader;
use goblin::elf64::sym;
use goblin::elf64::{header::*, program_header::*, reloc::Rela, section_header::*};
use log::{info, warn};
use plain::Plain;

use crate::arch::start;
use crate::entity::Tls;

pub struct KernelELF<'a> {
    pub elf: &'a [u8],
    pub header: &'a Header,
    pub phs: &'a [ProgramHeader],
    pub shs: &'a [SectionHeader],
    // relas: &'a [Rela],
}

impl<'a> KernelELF<'a> {
    pub fn parse(elf: &'a [u8]) -> KernelELF<'a> {
        let header = plain::from_bytes::<Header>(elf).unwrap();
        // dbg!(header);

        let phs = ProgramHeader::slice_from_bytes_len(
            &elf[header.e_phoff as usize..],
            header.e_phnum as usize,
        )
        .unwrap();
        // dbg!(phs);

        let shs = SectionHeader::slice_from_bytes_len(
            &elf[header.e_shoff as usize..],
            header.e_shnum as usize,
        )
        .unwrap();
        // dbg!(shs);

        if header.e_ident[EI_CLASS] != ELFCLASS64 {
            // dbg!("ELF_CLASS = {}", header.e_ident[EI_CLASS]);
            panic!("bootloader only supports 64bit kernel")
        } else if header.e_ident[EI_DATA] != ELFDATA2LSB {
            // dbg!("ELF_DATA = {}", header.e_ident[EI_DATA]);
            panic!("bootloader only supports little endian kernel")
        } else if header.e_ident[EI_OSABI] != ELFOSABI_STANDALONE {
            warn!("ELF_OSABI = {}", header.e_ident[EI_OSABI]);
        }
        // let entry_point = header.e_entry;

        // let note_section = phs
        //     .iter()
        //     .find(|ph| ph.p_type == PT_NOTE)
        //     .ok_or(panic!("note segment not detected"));

        // let relas = Rela::slice_from_bytes_len(&elf[], len)

        KernelELF {
            elf,
            header,
            phs,
            shs,
            // relas: todo!(),
        }
    }

    pub fn start_addr(&self) -> u64 {
        self.phs.iter().find(|ph| ph.p_type == PT_LOAD).unwrap().p_vaddr
    }

    pub fn mem_range(&self) -> Range<usize> {
        let first_ph = self.phs.iter().find(|ph| ph.p_type == PT_LOAD).unwrap();
        let last_ph = self
            .phs
            .iter()
            .rev()
            .find(|ph| ph.p_type == PT_LOAD)
            .unwrap();

        first_ph.p_vaddr as usize..(last_ph.p_vaddr + last_ph.p_memsz) as usize
    }

    pub fn mem_size(&self) -> usize {
        let range = self.mem_range();
        range.end - range.start
    }

    pub fn load_kernel(&self, memory: &mut [MaybeUninit<u8>]) -> Kernel {
        let memory_ptr = memory.as_ptr() as u64;
        let ph_range: Range<usize> = self.mem_range();
        // PT_LOAD
        self.phs
            .iter()
            .filter(|ph| ph.p_type == PT_LOAD)
            .for_each(|ph| {
                let ph_memory =
                    &mut memory[(ph.p_vaddr - ph_range.start as u64) as usize..][..ph.p_memsz as usize];
                let ph_file = &self.elf[ph.p_offset as usize..][..ph.p_filesz as usize];
                let ph_file: &[MaybeUninit<u8>] = unsafe { mem::transmute(ph_file) };
                ph_memory[..ph.p_filesz as usize].copy_from_slice(ph_file);
                for byte in &mut ph_memory[ph.p_filesz as usize..] {
                    byte.write(0);
                }
            });
        // PT_TLS
        let tls = self
            .phs
            .iter()
            .find(|ph| ph.p_type == PT_TLS)
            .map(|ph| Tls {
                start: ph.p_vaddr,
                filesz: ph.p_filesz,
                memsz: ph.p_memsz,
                align: ph.p_align,
            });


        info!("entry_point: {:#x}", memory_ptr);

        Kernel {
            entry_point: memory_ptr,
            kernel_addr_range: memory_ptr..memory_ptr + self.mem_size() as u64,
            tls,
        }
    }
}

pub struct Kernel {
    pub entry_point: u64,
    pub kernel_addr_range: Range<u64>,
    pub tls: Option<Tls>,
}
