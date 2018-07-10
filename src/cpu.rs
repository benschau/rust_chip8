use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;

static mut SCREEN: [[::BYTE; 32]; 64] = [[0; 32]; 64];

struct Cpu {
    game_mem: [::BYTE; 0xFFF],
    regs: [::BYTE; 16],
    address_regs: ::WORD,
    pc: ::WORD,
    m_stack: Vec<::WORD>,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            game_mem: [0; 0xFFF],
            regs: [0; 16],
            address_regs: 0,
            pc: 0x200,
            m_stack: Vec::new(),
        }
    }
}

impl Cpu {
    fn new(filepath: &str) -> Cpu {
        let mut file = File::create(filepath).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();
        let mem = Cpu::init_mem(&contents.into_bytes());
        
        Cpu {
            game_mem: mem,
            regs: [0; 16],
            address_regs: 0,
            pc: 0x200,
            m_stack: Vec::new(),
        }
    } 

    fn init_mem(bytes: &[::BYTE]) -> [::BYTE; 0xFFF] {
        let mut mem = [0; 0xFFF];
        let bytes = &bytes[..0xFFF];
        
        { 
            let (_left, right) = mem.split_at_mut(0x200);
            right.clone_from_slice(&bytes[..0xFFF]);
        }

        mem
    }

    fn get_opcode(&mut self) -> ::WORD {
        let mut opcode = self.game_mem[self.pc as usize] as ::WORD;
        opcode <<= 8; 
        opcode |= self.game_mem[(self.pc + 1) as usize] as ::WORD;
        self.pc += 2;
        
        opcode
    }

    fn decode_opcode(optcode: ::WORD) {
    
    }
    
    ///
    /// 0NNN - call RCA 1802 program at address NNN
    ///
    fn opcode_0nnn(&mut self, opcode: ::WORD) {
        self.pc = opcode & 0x0FFF; 
    }
    
    ///
    /// 00E0 - clear screen
    ///
    fn opcode_00e0(&mut self, _opcode: ::WORD) {
        // SCREEN = [[0; 32]; 64];
    }
    
    ///
    /// 00EE - return from subroutine
    ///
    fn opcode_00ee(&mut self, _opcode: ::WORD) {
        self.pc = self.m_stack.pop().unwrap();
    }
    
    ///
    /// 1NNN - jump to adress NNN.
    ///
    fn opcode_1nnn(&mut self, opcode: ::WORD) {
        self.pc = opcode & 0x0FFF; 
    }

    ///
    /// 2NNN - call subroutine at NNN.
    ///
    fn opcode_2nnn(&mut self, opcode: ::WORD) {
        self.m_stack.push(self.pc);
        self.pc = opcode & 0x0FFF;
    }

    ///
    /// 3XNN - skip next instruction if VX == NN.
    ///
    fn opcode_3xnn(&mut self, opcode: ::WORD) {
        let regx = self.regs[((opcode >> 8) & 0x0F) as usize] as ::BYTE;
        let val = (opcode & 0x00FF) as ::BYTE;

        if regx == val {
            self.pc = self.pc + 1;
        }
    }

    ///
    /// 4XNN - skip next instruction if VX != NN
    ///
    fn opcode_4xnn(&mut self, opcode: ::WORD) {
        let regx = self.regs[((opcode >> 8) & 0x0F) as usize] as ::BYTE;
        let val = (opcode & 0x00FF) as ::BYTE;

        if regx != val {
            self.pc = self.pc + 1; 
        }
    }

    ///
    /// 5XY0 - skip next instruction if VX == VY
    ///
    fn opcode_5xy0(&mut self, opcode: ::WORD) {
        let regx = self.regs[((opcode >> 8) & 0x0F) as usize] as ::BYTE;
        let regy = self.regs[((opcode >> 4) & 0x0F) as usize] as ::BYTE;

        if regx == regy {
            self.pc = self.pc + 1;
        }
    }

    ///
    /// 6XNN - set VX to NN
    ///
    fn opcode_6xnn(&mut self, opcode: ::WORD) {
        let regx_loc = ((opcode >> 8) & 0x0F) as usize;
        let val = (opcode & 0x00FF) as ::BYTE;

        self.regs[regx_loc] = val; 
    }

    ///
    /// 7XNN - adds NN to VX
    ///
    fn opcode_7xnn(&mut self, opcode: ::WORD) {
        let regx_loc = ((opcode >> 8) & 0x0F) as usize;
        let val = (opcode & 0x00FF) as ::BYTE;
        
        self.regs[regx_loc] = self.regs[regx_loc] + val;
    }
}


