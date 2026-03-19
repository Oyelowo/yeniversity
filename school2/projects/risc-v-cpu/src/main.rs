//! RISC-V RV32I soft CPU — fetch-decode-execute emulator.
//!
//! Supports the full RV32I base integer instruction set:
//! R-type, I-type, S-type, B-type, U-type, J-type, FENCE (nop), ECALL/EBREAK.
//!
//! Usage: run with a hardcoded test program; extend to load ELF binaries.

mod cpu;
use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new(64 * 1024); // 64 KiB RAM

    // Example program: compute 1+2+3+...+10 = 55 and store in x10
    // li x1, 10    (addi x1, x0, 10)
    // li x2, 0     (addi x2, x0, 0)   -- accumulator
    // li x3, 0     (addi x3, x0, 0)   -- counter
    // loop:
    //   addi x3, x3, 1
    //   add  x2, x2, x3
    //   bne  x3, x1, loop
    // (x2 should = 55)

    let instrs: &[u32] = &[
        0x00a00093, // addi x1, x0, 10
        0x00000113, // addi x2, x0, 0
        0x00000193, // addi x3, x0, 0
        // loop (PC=12):
        0x00118193, // addi x3, x3, 1
        0x00310133, // add  x2, x2, x3
        0xfe1190e3, // bne  x3, x1, -16 (back to loop start at PC=12)
    ];

    cpu.load_program(instrs, 0);
    cpu.run(200); // run up to 200 steps

    println!("Register x2 (sum) = {} (expected 55)", cpu.regs[2]);
    assert_eq!(cpu.regs[2], 55);
    println!("RISC-V RV32I sum 1..10 PASSED ✓");
}
