#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub show_addr: bool,
    pub show_hex: bool,
    pub resolve_rel: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            show_addr: true,
            show_hex: true,
            resolve_rel: true,
        }
    }
}

#[derive(Debug)]
pub struct Spc {
    pub mem: [u8; 65536],
    pub pc: u16,
    pub stop: u16,
    pub config: Config,
}

impl Spc {
    pub fn new(config: Config, start_pc: u16, stop_addr: u16) -> Self {
        Spc {
            mem: [0xFF; 65536],
            pc: start_pc,
            stop: stop_addr,
            config,
        }
    }

    pub fn load_rom(&mut self, data: &[u8], load_addr: u16) {
        let start = load_addr as usize;
        if start >= self.mem.len() {
            return;
        }
        let end = (start + data.len()).min(self.mem.len());
        let len_to_copy = end - start;
        self.mem[start..end].copy_from_slice(&data[..len_to_copy]);
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let lo = self.read_byte(addr);
        let hi = self.read_byte(addr.wrapping_add(1));
        u16::from_le_bytes([lo, hi])
    }
}
