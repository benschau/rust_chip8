extern crate rand;
extern crate piston_window;

use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;
use cpu::rand::prelude::*;
use font::FONT;
use self::piston_window::*;

const DELAY_FREQ: ::BYTE = 60;
const SOUND_FREQ: ::BYTE = 60;
// screen_scaling here means that every 'pixel' is screen_scaling x screen_scaling size.
const SCREEN_SCALING: u32 = 10;
const SCREEN_DIM: (u32, u32) = (64 * SCREEN_SCALING, 32 * SCREEN_SCALING);

// our screen frame buffer
static mut SCREEN: [[::BYTE; SCREEN_DIM.0 as usize]; SCREEN_DIM.1 as usize] = 
                   [[0; SCREEN_DIM.0 as usize]; SCREEN_DIM.1 as usize];

static mut KEYBOARD: [bool; 16] = [false; 16];
static mut DELAY_TIMER: ::BYTE = DELAY_FREQ;
static mut SOUND_TIMER: ::BYTE = SOUND_FREQ;

pub struct Cpu {
    game_mem: [::BYTE; 0xFFF],
    regs: [::BYTE; 16],
    addr_reg: ::WORD,
    pc: ::WORD,
    m_stack: Vec<::WORD>,
}

pub enum CpuError {
    IncorrectFilePath,
    UnReadableFile,
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
    pub fn new(filepath: &str) -> Result<Cpu, CpuError> {
        // TODO: add better exception handling
        let mut file = match File::open(filepath) {
            Err(why) => return Err(CpuError::IncorrectFilePath),
            Ok(file) => file,
        };

        let mut contents: Vec<u8> = Vec::new();

        file.read_to_end(&mut contents);
        let mut mem = Cpu::init_mem(contents);
        Cpu::init_font(&mut mem);
        
        Ok(Cpu {
            game_mem: mem,
            regs: [0; 16],
            addr_reg: 0,
            pc: 0x200,
            m_stack: Vec::new(),
        })
    }
    
    fn init_mem(bytes: Vec<u8>) -> [::BYTE; 0xFFF] {
        let mut mem = [0; 0xFFF];
        let len = bytes.len();
        
        { 
            let (_left, right) = mem.split_at_mut(0x200);
            right[..len].clone_from_slice(&bytes[..len]);
        } 

        mem
    }
    
    ///
    /// init_font - load chip8 fontset into the game_mem[0x50] onward.
    ///
    fn init_font(bytes: &mut [::BYTE]) {
        for i in 0..80 as usize {
            bytes[i + 0x50] = FONT[i];
        }
    }
    
    ///
    /// run - Read the contents of game memory, fetch, decode, execute opcode instructions.
    ///
    pub fn run(&mut self) {
        let mut window: PistonWindow = 
            WindowSettings::new("rust-chip8", SCREEN_DIM)
                .exit_on_esc(true)
                .build()
                .unwrap();

        // TODO: Add a menu bar at the top 
        while let Some(event) = window.next() {
            window.draw_2d(&event, |context, graphics| {
                clear([1.0; 4], graphics);
                rectangle([1.0, 0.0, 0.0, 1.0],
                          [0.0, 0.0, 100.0, 100.0],
                          context.transform,
                          graphics);
            });
        }
    }
    
    ///
    /// get_opcode - fetch the opcode that the pc is looking at.
    ///
    fn get_opcode(&mut self) -> ::WORD {
        let mut opcode = self.game_mem[self.pc as usize] as ::WORD;
        opcode <<= 8; 
        opcode |= self.game_mem[(self.pc + 1) as usize] as ::WORD;
        self.pc += 2;
        
        opcode
    }
    
    ///
    /// decode_opcode - decode the given opcode using the following structure:
    ///
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
        let regx = self.regs[((opcode >> 8) & 0x0F) as usize];
        let regy = self.regs[((opcode >> 4) & 0x00F) as usize];
        let coord = (regx, regy);
        let height = opcode & 0x000F;
        
        self.regs[0xF] = 0;
        
        unsafe {
            for line in 0..height {
                let data: ::BYTE = self.game_mem[(self.addr_reg + line) as usize];
                let xpixel_bit = 7;
                let xpixel = 0;
                for xpixel in 0..8 {
                    let mask = 1 << xpixel_bit;
                    if (data & mask) == 1 {
                        let x: usize = (coord.0 + xpixel) as usize;
                        let y: usize = (coord.1 + line as ::BYTE) as usize;
                        if SCREEN[x][y] == 1 {
                            self.regs[0xF] = 1; 
                        }

                        SCREEN[x][y] ^= 1;
                    }
                }
            }
        }
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

        // TODO: FILL OUT
         
    }
    
    ///
    /// fx15 - set delay timer to VX.
    ///
    fn opcode_fx15(&mut self, opcode: ::WORD) {
        let regx = self.regs[((opcode >> 8) & 0x0F) as usize];
        
        unsafe {
            DELAY_TIMER = regx;
        }
    }

    ///
    /// fx18 - set sound timer to VX.
    ///
    fn opcode_fx18(&mut self, opcode: ::WORD) {
        let regx = self.regs[((opcode >> 8) & 0x0F) as usize];
        
        unsafe {
            SOUND_TIMER = regx;
        }
    }

    ///
    /// fx1e - add VX to I (addr reg).
    ///
    fn opcode_fx1e(&mut self, opcode: ::WORD) {
        let regx = self.regs[((opcode >> 8) & 0x0F) as usize] as ::WORD;
            
        self.addr_reg += regx;
    }

    ///
    /// fx29 - sets I (addr reg) to the location of the sprite for the character (0-F) in VX
    ///        (characters are represented as a 4x5 font)
    ///        e.g I = sprite_addr[VX]
    ///        
    fn opcode_fx29(&mut self, opcode: ::WORD) {
        let regx = self.regs[((opcode >> 8) & 0x0F) as usize];
        
        self.addr_reg = (0x50 + (regx * 5)).into();
    }

    ///
    /// fx33 - stores the binary-coded decimal representation of VX with the most significant of
    /// three digits at the address in I, the middle digit at I + 1, least significant at i + 2.
    ///
    fn opcode_fx33(&mut self, opcode: ::WORD) {
        let regx = self.regs[((opcode >> 8) & 0x0F) as usize];
        
        self.game_mem[(self.addr_reg as usize) + 2] = regx >> 5;
        self.game_mem[(self.addr_reg as usize) + 1] = (regx >> 4) & 3;
        self.game_mem[self.addr_reg as usize] = regx & 3;
    }

    ///
    /// fx55 - stores V0 to VX, inclusive, in memory starting from address @ I. The offset from I
    /// is increased by 1 for each value written, but I itself is unmodified.
    ///
    fn opcode_fx55(&mut self, opcode: ::WORD) {
        let regx = self.regs[((opcode >> 8) & 0x0F) as usize] as usize;
        
        for i in 0..(regx + 1) as usize {
            self.game_mem[(self.addr_reg as usize) + i] = self.regs[i]; 
        }
    }

    ///
    /// fx65 - fills V0 to VX, inclusive, with values from address @ I. Offset from I for each
    /// value written, but I itself is unmodified.
    ///
    fn opcode_fx65(&mut self, opcode: ::WORD) {
        let regx = self.regs[((opcode >> 8) & 0x0F) as usize] as usize;
        
        for i in 0..(regx + 1) as usize {
                self.regs[i] = self.game_mem[(self.addr_reg as usize) + i];
        }
    }
}


