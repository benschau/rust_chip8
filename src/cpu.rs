extern crate rand;

use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;
use cpu::rand::prelude::*;

const DELAY_FREQ: ::BYTE = 60;
const SOUND_FREQ: ::BYTE = 60;

static mut SCREEN: [[::BYTE; 32]; 64] = [[0; 32]; 64];
static mut KEYBOARD: [bool; 16] = [false; 16];
static mut DELAY_TIMER: ::BYTE = DELAY_FREQ;
static mut SOUND_TIMER: ::BYTE = SOUND_FREQ;

struct Cpu {
    game_mem: [::BYTE; 0xFFF],
    regs: [::BYTE; 16],
    addr_reg: ::WORD,
    pc: ::WORD,
    m_stack: Vec<::WORD>,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            game_mem: [0; 0xFFF],
            regs: [0; 16],
            addr_reg: 0,
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
            addr_reg: 0,
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
            self.pc += 1;
        }
    }

    ///
    /// 4XNN - skip next instruction if VX != NN
    ///
    fn opcode_4xnn(&mut self, opcode: ::WORD) {
        let regx = self.regs[((opcode >> 8) & 0x0F) as usize] as ::BYTE;
        let val = (opcode & 0x00FF) as ::BYTE;

        if regx != val {
            self.pc += 1; 
        }
    }

    ///
    /// 5XY0 - skip next instruction if VX == VY
    ///
    fn opcode_5xy0(&mut self, opcode: ::WORD) {
        let regx = self.regs[((opcode >> 8) & 0x0F) as usize] as ::BYTE;
        let regy = self.regs[((opcode >> 4) & 0x0F) as usize] as ::BYTE;

        if regx == regy {
            self.pc += 1;
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
    
    // TODO: Compress 8xy series into one function
    ///
    /// 8XY0 - set VX to VY
    ///
    fn opcode_8xy0(&mut self, opcode: ::WORD) {
        let regx_loc = ((opcode >> 8) & 0x0F) as usize;
        let regy = self.regs[((opcode >> 4) & 0x00F) as usize];
    
        self.regs[regx_loc] = regy; 
    }

    ///
    /// 8XY1 - set VX to VX | VY
    ///
    fn opcode_8xy1(&mut self, opcode: ::WORD) {
        let regx_loc = ((opcode >> 8) & 0x0F) as usize;
        let regy = self.regs[((opcode >> 4) & 0x00F) as usize];
        
        self.regs[regx_loc] = self.regs[regx_loc] | regy;
    }

    ///
    /// 8xy2 - set VX to VX & VY
    ///
    fn opcode_8xy2(&mut self, opcode: ::WORD) {
        let regx_loc = ((opcode >> 8) & 0x0F) as usize;
        let regy = self.regs[((opcode >> 4) & 0x00F) as usize];
        
        self.regs[regx_loc] = self.regs[regx_loc] & regy;
    }
    
    ///
    /// 8xy3 - set VX to VX ^ VY
    ///
    fn opcode_8xy3(&mut self, opcode: ::WORD) {
        let regx_loc = ((opcode >> 8) & 0x0F) as usize;
        let regy = self.regs[((opcode >> 4) & 0x00F) as usize];
        
        self.regs[regx_loc] = self.regs[regx_loc] ^ regy;
    }

    ///
    /// 8xy4 - add VY to Vx 
    ///
    fn opcode_8xy4(&mut self, opcode: ::WORD) {
        let regx_loc = ((opcode >> 8) & 0x0F) as usize;
        let regy = self.regs[((opcode >> 4) & 0x00F) as usize];
        
        self.regs[regx_loc] = self.regs[regx_loc] + regy; 
    }

    ///
    /// 8xy5 - subtract VY from VX
    ///
    fn opcode_8xy5(&mut self, opcode: ::WORD) {
        let regx_loc = ((opcode >> 8) & 0x0F) as usize;
        let regx = self.regs[regx_loc];
        let regy = self.regs[((opcode >> 4) & 0x00F) as usize];
       
        self.regs[0xF] = (regy > regx) as ::BYTE; 
        self.regs[regx_loc] = regx - regy; 
    }

    ///
    /// 8xy6 - shift VY << 1, store to VX. Set VF to value of least significant bit of VY prior to
    /// shift.
    ///
    fn opcode_8xy6(&mut self, opcode: ::WORD) {
        let regx_loc = ((opcode >> 8) & 0x0F) as usize;
        let regy = self.regs[((opcode >> 4) & 0x00F) as usize];
        
        self.regs[0xF] = regy & 1;
        self.regs[regx_loc] = regy << 1;
    }

    ///
    /// 8xy7 - set VX to VY minux VX; VF set to 0 if borrow, 1 if not.
    ///
    fn opcode_8xy7(&mut self, opcode: ::WORD) {
        let regx_loc = ((opcode >> 8) & 0x0F) as usize;
        let regx = self.regs[regx_loc];
        let regy = self.regs[((opcode >> 4) & 0x00F) as usize];

        self.regs[0xF] = (regy > regx) as ::BYTE;
        self.regs[regx_loc] = regy - regx;
    }

    ///
    /// 8xye - shifts VY left by one, copies result to VX; VF set to value of most significant bit
    /// of VY before shift.
    ///
    fn opcode_8xye(&mut self, opcode: ::WORD) {
        let regx_loc = ((opcode >> 8) & 0x0F) as usize;
        let regy = self.regs[((opcode >> 4) & 0x00F) as usize];
        
        self.regs[0xF] = regy & 2_u8.pow(15);
        self.regs[regx_loc] = regy << 1;
    }

    ///
    /// 9xy0 - skips the next instr if VX != VY
    ///
    fn opcode_9xy0(&mut self, opcode: ::WORD) {
        let regx_loc = ((opcode >> 8) & 0x0F) as usize;
        let regx = self.regs[regx_loc];
        let regy = self.regs[((opcode >> 4) & 0x00F) as usize];
        
        if regx != regy {
            self.pc += 1;
        }
    }

    ///
    /// annn - sets I (address reg) to the address NNN
    ///
    fn opcode_annn(&mut self, opcode: ::WORD) {
        let addr = opcode & 0x0FFF;
        self.addr_reg = addr;
    }

    ///
    /// bnnn - jump to address NNN + V0
    ///
    fn opcode_bnnn(&mut self, opcode: ::WORD) {
        let addr = opcode & 0x0FFF;
        self.pc = (self.regs[0] as ::WORD) + addr;
    }

    ///
    /// cxnn - set VX to the result of a bitwise and operation on a random number (0 - 255) and NN
    ///
    fn opcode_cxnn(&mut self, opcode: ::WORD) {
        let val = (opcode & 0x00FF) as ::BYTE;
        let regx_loc = ((opcode >> 8) & 0x0F) as usize;
        let rng: u8 = thread_rng().gen();

        self.regs[regx_loc] = rng & val;
    }

    ///
    /// dxyn - draw a sprite at (VX, VY), width of 8 pixels and height of N pixels. each row of
    /// eight pixels read as bit-coded starting from I (address reg). VF to 1 if any pixels set to
    /// unset when sprite is drawn (e.g some pixel is overwritten), 0 if not.
    ///
    fn opcode_dxyn(&mut self, opcode: ::WORD) {
        
    }

    ///
    /// ex9e - skip next instruction if the key stored in VX is pressed.
    ///
    fn opcode_ex9e(&mut self, opcode: ::WORD) {
        let key = self.regs[((opcode >> 8) & 0x0F) as usize] as usize;
        
        unsafe {
            if KEYBOARD[key] {
                self.pc += 1; 
            }
        }
    }

    ///
    /// exa1 - skip next instruction if the key stored in VX isn't pressed.
    ///
    fn opcode_exa1(&mut self, opcode: ::WORD) {
        let key = self.regs[((opcode >> 8) & 0x0F) as usize] as usize;
        
        unsafe {
            if !KEYBOARD[key] {
                self.pc += 1; 
            }
        }
    }

    ///
    /// fx07 - sets VX to the value of the delay timer.
    ///
    fn opcode_fx07(&mut self, opcode: ::WORD) {
        let regx_loc = ((opcode >> 8) & 0x0F) as usize;
        
        unsafe {
            self.regs[regx_loc] = DELAY_TIMER;      
        }
    }

    ///
    /// fx0a - a key press is awaited, then store in VX.
    ///        (blocks all other instructions until next key event)
    ///
    fn opcode_fx0a(&mut self, opcode: ::WORD) {
        let regx_loc = ((opcode >> 8) & 0x0F) as usize;
        
    }
}


