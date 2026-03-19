//! RV32I CPU core: registers, memory, fetch-decode-execute.

pub struct Cpu {
    pub regs: [i32; 32],  // x0..x31 (x0 hardwired to 0)
    pub pc: u32,
    pub mem: Vec<u8>,
    pub halted: bool,
}

impl Cpu {
    pub fn new(mem_size: usize) -> Self {
        Self { regs: [0; 32], pc: 0, mem: vec![0; mem_size], halted: false }
    }

    pub fn load_program(&mut self, instrs: &[u32], base_addr: usize) {
        for (i, &instr) in instrs.iter().enumerate() {
            let addr = base_addr + i * 4;
            self.mem[addr]   = (instr & 0xFF) as u8;
            self.mem[addr+1] = ((instr >> 8) & 0xFF) as u8;
            self.mem[addr+2] = ((instr >> 16) & 0xFF) as u8;
            self.mem[addr+3] = ((instr >> 24) & 0xFF) as u8;
        }
    }

    fn fetch(&self) -> u32 {
        let a = self.pc as usize;
        (self.mem[a] as u32)
            | ((self.mem[a+1] as u32) << 8)
            | ((self.mem[a+2] as u32) << 16)
            | ((self.mem[a+3] as u32) << 24)
    }

    fn read_mem32(&self, addr: u32) -> i32 {
        let a = addr as usize;
        (self.mem[a] as i32)
            | ((self.mem[a+1] as i32) << 8)
            | ((self.mem[a+2] as i32) << 16)
            | ((self.mem[a+3] as i32) << 24)
    }

    fn write_mem32(&mut self, addr: u32, val: i32) {
        let a = addr as usize;
        self.mem[a]   = (val & 0xFF) as u8;
        self.mem[a+1] = ((val >> 8) & 0xFF) as u8;
        self.mem[a+2] = ((val >> 16) & 0xFF) as u8;
        self.mem[a+3] = ((val >> 24) & 0xFF) as u8;
    }

    fn sign_extend(val: u32, bits: u32) -> i32 {
        let shift = 32 - bits;
        ((val << shift) as i32) >> shift
    }

    fn set_reg(&mut self, rd: usize, val: i32) {
        if rd != 0 { self.regs[rd] = val; }
    }

    pub fn step(&mut self) {
        if self.halted { return; }
        let instr = self.fetch();
        let opcode = instr & 0x7F;
        let rd     = ((instr >> 7)  & 0x1F) as usize;
        let funct3 = (instr >> 12) & 0x7;
        let rs1    = ((instr >> 15) & 0x1F) as usize;
        let rs2    = ((instr >> 20) & 0x1F) as usize;
        let funct7 = (instr >> 25) & 0x7F;

        let imm_i = Self::sign_extend((instr >> 20) & 0xFFF, 12);
        let imm_s = Self::sign_extend(((instr >> 25) << 5) | ((instr >> 7) & 0x1F), 12);
        let imm_b = Self::sign_extend(
            ((instr >> 31) << 12) | (((instr >> 7) & 1) << 11)
            | (((instr >> 25) & 0x3F) << 5) | (((instr >> 8) & 0xF) << 1), 13);
        let imm_u = (instr & 0xFFFFF000) as i32;
        let imm_j = Self::sign_extend(
            ((instr >> 31) << 20) | (((instr >> 12) & 0xFF) << 12)
            | (((instr >> 20) & 1) << 11) | (((instr >> 21) & 0x3FF) << 1), 21);

        let pc = self.pc as i32;
        let r1 = self.regs[rs1];
        let r2 = self.regs[rs2];
        let mut next_pc = self.pc.wrapping_add(4);

        match opcode {
            // LUI
            0x37 => self.set_reg(rd, imm_u),
            // AUIPC
            0x17 => self.set_reg(rd, pc.wrapping_add(imm_u)),
            // JAL
            0x6F => { self.set_reg(rd, pc + 4); next_pc = pc.wrapping_add(imm_j) as u32; }
            // JALR
            0x67 => { self.set_reg(rd, pc + 4); next_pc = (r1.wrapping_add(imm_i) & !1) as u32; }
            // Branches
            0x63 => {
                let taken = match funct3 {
                    0x0 => r1 == r2,
                    0x1 => r1 != r2,
                    0x4 => r1 < r2,
                    0x5 => r1 >= r2,
                    0x6 => (r1 as u32) < (r2 as u32),
                    0x7 => (r1 as u32) >= (r2 as u32),
                    _ => false,
                };
                if taken { next_pc = pc.wrapping_add(imm_b) as u32; }
            }
            // Loads
            0x03 => {
                let addr = (r1.wrapping_add(imm_i)) as u32;
                let val = match funct3 {
                    0x0 => self.mem[addr as usize] as i8 as i32,
                    0x1 => (self.mem[addr as usize] as i32) | ((self.mem[addr as usize+1] as i32) << 8),
                    0x2 => self.read_mem32(addr),
                    0x4 => self.mem[addr as usize] as i32,
                    _ => 0,
                };
                self.set_reg(rd, val);
            }
            // Stores
            0x23 => {
                let addr = (r1.wrapping_add(imm_s)) as u32;
                match funct3 {
                    0x0 => self.mem[addr as usize] = (r2 & 0xFF) as u8,
                    0x1 => { self.mem[addr as usize] = (r2 & 0xFF) as u8; self.mem[addr as usize+1] = ((r2>>8)&0xFF) as u8; }
                    0x2 => self.write_mem32(addr, r2),
                    _ => {}
                }
            }
            // OP-IMM
            0x13 => {
                let val = match funct3 {
                    0x0 => r1.wrapping_add(imm_i),
                    0x1 => (r1 as u32).wrapping_shl(imm_i as u32 & 31) as i32,
                    0x2 => (r1 < imm_i) as i32,
                    0x3 => ((r1 as u32) < imm_i as u32) as i32,
                    0x4 => r1 ^ imm_i,
                    0x5 => if funct7 == 0x20 { r1 >> (imm_i & 31) } else { ((r1 as u32) >> (imm_i as u32 & 31)) as i32 },
                    0x6 => r1 | imm_i,
                    0x7 => r1 & imm_i,
                    _ => 0,
                };
                self.set_reg(rd, val);
            }
            // OP (register-register)
            0x33 => {
                let val = match (funct3, funct7) {
                    (0x0, 0x00) => r1.wrapping_add(r2),
                    (0x0, 0x20) => r1.wrapping_sub(r2),
                    (0x1, _)    => (r1 as u32).wrapping_shl(r2 as u32 & 31) as i32,
                    (0x2, _)    => (r1 < r2) as i32,
                    (0x3, _)    => ((r1 as u32) < (r2 as u32)) as i32,
                    (0x4, _)    => r1 ^ r2,
                    (0x5, 0x00) => ((r1 as u32).wrapping_shr(r2 as u32 & 31)) as i32,
                    (0x5, 0x20) => r1 >> (r2 & 31),
                    (0x6, _)    => r1 | r2,
                    (0x7, _)    => r1 & r2,
                    _ => 0,
                };
                self.set_reg(rd, val);
            }
            // ECALL / EBREAK → halt
            0x73 => { self.halted = true; }
            // FENCE → nop
            0x0F => {}
            _ => { eprintln!("Unknown opcode 0x{:02X} at PC={}", opcode, self.pc); self.halted = true; }
        }
        self.pc = next_pc;
    }

    pub fn run(&mut self, max_steps: usize) {
        for _ in 0..max_steps {
            if self.halted { break; }
            self.step();
        }
    }
}
