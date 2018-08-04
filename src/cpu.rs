extern crate rand;

use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;
use cpu::rand::prelude::*;
use font::FONT;

const DELAY_FREQ: ::BYTE = 60;
const SOUND_FREQ: ::BYTE = 60;
static mut SCREEN: [[::BYTE; 32]; 64] = [[0; 32]; 64];
static mut KEYBOARD: [bool; 16] = [false; 16];
static mut DELAY_TIMER: ::BYTE = DELAY_FREQ;
static mut SOUND_TIMER: ::BYTE = SOUND_FREQ;

struct OpcodeByte<'a> {
    curr_opcode: ::WORD,
    func: Option<&'a FnMut(&mut Cpu, ::WORD)>,
    suffix_bits: Vec<&'a OpcodeByte<'a>>
}

impl<'a> OpcodeByte<'a> {

    ///
    /// new - create a new opcode bit structure.
    ///
    /// The opcode byte structure is meant to keep track of the current decoded opcode. 
    /// We split the opcode down at the index given, such that given the word and index: 
    ///     0xDXYN, 1
    /// is split so that curr_opcode = D, and the suffix_bits are set to the possible bits for the
    /// next opcode, e.g X, or nothing if the only choice is DXYN (which it is). Since DXYN is the
    /// leaf in our tree, we point func toward Cpu::opcode_dxyn() for the decoder tree.
    ///     0xDXYN, 2 => curr_opcode = DX, suffix_bits = 
    ///
    fn new(opcode: ::WORD, 
           index: ::BYTE, 
           cpu_fp: Option<&'a FnMut(&mut Cpu, ::WORD)>) -> OpcodeByte<'a> {
        
        // TODO: Shift curr_opcode and suffix bits to create vector of opcode bytes and root
        OpcodeByte {
            curr_opcode: opcode,
            func: cpu_fp,
            suffix_bits: Vec::new(),
        }
    }
}

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
        let mut mem = Cpu::init_mem(&contents.into_bytes());
        Cpu::init_font(&mut mem);
        
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
    
    ///
    /// init_decoder - Creates the following structure for later opcode decomp:
    /// 
    /// The first array of pointers is points either to another array (if there are more
    /// possibilities) or to the actual function that the opcode represents. We traverse this
    /// structure to the bottom, like a tree, and the leaf is the interpreted CPU opcode that is run. 
    /// 
    /// Each subarray of pointers is representative of the next bit needed to be interpreted,
    /// or the next distinctive bit- e.g for the opcodes that begin with 8, we can skip the
    /// next two bits XY and we base the final opcode on the last bit (that we use to
    /// differentiate against the other opcodes that begin with 8).
    ///
    ///     [ 0, 1NNN, 2NNN, 3XNN, 4XNN, 5XY0, 6XNN, 7XNN, 8, 9XY0, ANNN, BNNN, CXNN, DXYN, ...
    ///       |                                            |                                
    ///      / \                                    [8XY0, 8XY1, ...]                   
    ///   [ E , 0NNN ]
    ///     |   
    ///     |   
    ///     |__
    ///    /    \
    ///  [00E0, 00EE]
    ///
    ///     [ E,                F ]
    ///       |                 | 
    ///     [EX(9E), EX(A1)]   [FX(07), FX(0A), FX(15), FX(18), FX(1E), FX(29), FX(33), FX(55),
    ///                         FX(65)]
    ///
    fn init_decoder() -> [::BYTE; 16] {
        let arr: [::BYTE; 16] = [0; 16];

        arr
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
    /// This structure should already have been created prior to the calling of this function.
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


