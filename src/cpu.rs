use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;

type BYTE = u8;
type WORD = u16;

struct cpu {
    game_mem: [BYTE; 0xFFF],
    regs: [BYTE; 16],
    address_regs: WORD,
    pc: WORD,
    m_stack: Vec<WORD>,
}

impl Default for cpu {
    fn default() -> Self {
        cpu {
            game_mem: [0; 0xFFF],
            regs: [0; 16],
            address_regs: 0,
            pc: 0x200,
            m_stack: Vec::new(),
        }
    }
}

impl cpu {
    fn new(filepath: &str) -> cpu {
        let mut file = File::create(filepath).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        let mem = cpu::init_mem(&contents.into_bytes());
        
        cpu {
            game_mem: mem,
            regs: [0; 16],
            address_regs: 0,
            pc: 0x200,
            m_stack: Vec::new(),
        }
    } 

    fn init_mem(bytes: &[BYTE]) -> [BYTE; 0xFFF] {
        let mut mem = [0; 0xFFF];

        
    }
}
