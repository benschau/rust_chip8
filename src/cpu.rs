using std::vec::Vec;

type BYTE = u8;
type WORD = u16;

struct CPU {
    game_mem: [BYTE; 0xFFF],
    regs: [BYTE; 16],
    address_regs: WORD,
    pc: WORD,
    m_stack: Vec<WORD>,
}

impl CPU {
    fn new() -> CPU {
        CPU {
             
        }
    }
}
