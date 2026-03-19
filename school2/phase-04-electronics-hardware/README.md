# Phase 04 — Electronics & Hardware

**Duration:** 3–5 months  
**Prerequisites:** Phase 02 (electromagnetism), Phase 03 (semiconductor physics)  
**Runs alongside:** Begin Phase 03 last section (MOSFET) immediately before this

---

## Why This Phase Exists

This is where physics becomes devices. Every computation ever performed, every signal ever
transmitted, every sensor ever read — it all happens in circuits. This phase builds the bridge
from semiconductor physics to real systems: you will understand why a transistor amplifies,
how to design a filter, what a CPU is at the gate level, and how to write firmware that talks
directly to hardware. By the end you can design a PCB, write bare-metal code, and describe
the architecture of a processor.

---

## Learning Objectives

- [ ] Analyse DC and AC circuits using Kirchhoff's laws, Thévenin/Norton equivalents, superposition
- [ ] Design and analyse amplifier circuits using BJTs and MOSFETs (small-signal models)
- [ ] Design active and passive filters; understand their frequency response
- [ ] Understand op-amp circuits: inverting/non-inverting, integrator, differentiator, comparator, instrumentation amplifier
- [ ] Apply the Fourier transform, Laplace transform, and Z-transform to signals and systems analysis
- [ ] Design FIR and IIR digital filters; understand sampling and aliasing (Nyquist-Shannon theorem)
- [ ] Design and analyse digital combinational and sequential logic from gates to complex systems
- [ ] Understand computer architecture: datapath, ALU, registers, memory, instruction set (RISC-V)
- [ ] Write bare-metal firmware in Rust (or C) for a microcontroller without OS or HAL
- [ ] Interface with hardware peripherals: UART, SPI, I2C, ADC, PWM, timers, DMA

---

## Topics

### 1. Circuit Analysis

#### DC Circuits
- Ohm's law; Kirchhoff's current law (KCL) and voltage law (KVL)
- Node voltage method; mesh current method
- Superposition; Thévenin's and Norton's theorems
- Maximum power transfer
- Non-ideal sources; source loading

#### AC Circuits
- Phasors; complex impedance: $Z_R = R$, $Z_C = 1/j\omega C$, $Z_L = j\omega L$
- AC power: real, reactive, apparent power; power factor
- Series and parallel RLC circuits; resonance; quality factor Q
- Frequency response: Bode plots from transfer functions
- Two-port networks: Z, Y, h parameters; input/output impedance

### 2. Semiconductor Devices & Analogue Circuits

#### Diode Applications
- Rectifiers: half-wave, full-wave, bridge; ripple and filtering
- Zener regulation; clippers and clampers
- Schottky diodes; LEDs; photodiodes

#### BJT Amplifiers
- Small-signal model (hybrid-π)
- Common-emitter, common-base, common-collector (emitter follower) configurations
- Biasing: voltage divider bias; DC operating point
- Gain, input impedance, output impedance of each configuration

#### MOSFET Amplifiers
- Small-signal model; $g_m$ and $r_o$
- Common-source, common-gate, common-drain (source follower)
- CMOS inverter: static and dynamic power; switching threshold
- Current mirrors; differential pair

#### Operational Amplifiers
- Ideal op-amp model: infinite gain, infinite $Z_{in}$, zero $Z_{out}$
- Inverting and non-inverting amplifiers; gain-bandwidth product
- Difference and instrumentation amplifiers
- Integrator and differentiator; practical limitations (saturation, slew rate)
- Comparators; Schmitt trigger (hysteresis)
- Active filters: Sallen-Key, multiple feedback; Butterworth and Chebyshev approximations

#### Power Electronics (Overview)
- Switching regulators: buck, boost, buck-boost converters
- Gate drivers; MOSFET switching losses
- H-bridge for motor control

### 3. Signals & Systems

#### Continuous-Time Signals
- Signal classification: periodic, aperiodic, energy, power signals
- Convolution; impulse response; LTI systems
- Laplace transform: definition, region of convergence, properties
- Transfer functions; poles and zeros; BIBO stability
- Fourier series; Fourier transform; Parseval's theorem
- Bode plots from the transfer function; gain and phase margins

#### Discrete-Time Signals & Systems
- Sampling theorem (Nyquist-Shannon); aliasing; reconstruction
- Discrete-time convolution; difference equations
- Z-transform: definition, properties, inverse Z-transform
- Discrete Fourier Transform (DFT); FFT algorithm
- Digital filter design: FIR (windowing, Parks-McClellan) and IIR (bilinear transform from analogue prototypes)

### 4. Digital Design

#### Combinational Logic
- Boolean algebra; De Morgan's laws; canonical forms (SOP, POS)
- Karnaugh maps; logic minimisation
- Standard combinational circuits: multiplexer, demultiplexer, encoder, decoder, adder, ALU

#### Sequential Logic
- Latches and flip-flops: SR, D, JK, T; setup and hold times
- Registers and counters; shift registers
- Finite state machines (FSM): Mealy and Moore; state diagram to logic implementation
- Synchronous design; clock domains; metastability and synchronisers

#### Computer Architecture (RISC-V)
- Instruction set architecture: registers, instruction encoding, addressing modes (RISC-V RV32I)
- Datapath: fetch, decode, execute, memory access, write-back
- Control unit design (hardwired and microprogrammed)
- Pipelining: stages, hazards (data, control, structural), forwarding, branch prediction
- Memory hierarchy: cache (direct-mapped, set-associative, fully associative), DRAM, virtual memory
- I/O: memory-mapped I/O, polling, interrupts, DMA

### 5. Embedded Systems

#### Microcontroller Architecture
- CPU core, bus matrix, memory map, clock tree
- Peripherals: GPIO, UART, SPI, I2C, ADC, DAC, Timers, PWM, DMA, RTC
- Interrupt controller (NVIC on ARM Cortex-M); interrupt priorities; critical sections
- Power modes: run, sleep, deep-sleep, standby

#### Bare-Metal Firmware in Rust
- No OS, no HAL: write startup code, linker scripts, vector tables
- Memory-mapped register access in Rust: `unsafe` pointer writes, `volatile`
- Using `svd2rust`-generated PAC crates for safe register access
- Writing a UART driver from scratch
- Implementing a blocking SPI master and I2C master
- ADC sampling; DMA transfers; timer-triggered ADC
- FreeRTOS (or Embassy async Rust) for concurrent tasks

#### Practical Hardware
- Schematic capture and PCB layout (KiCad)
- Decoupling capacitors, power plane design, signal integrity basics
- Test equipment: oscilloscope, multimeter, logic analyser, spectrum analyser
- Signal probing and debugging techniques

---

## Core Texts

| Text | Notes |
|------|-------|
| **Horowitz & Hill — The Art of Electronics** (3rd ed.) | The practitioner's bible. Build the intuition from transistors up. Read cover-to-cover; do the worked examples on the bench. |
| **Sedra & Smith — Microelectronic Circuits** (8th ed.) | Rigorous treatment of BJT, MOSFET, op-amp circuits; strong on small-signal models and frequency response. |
| **Oppenheim & Willsky — Signals and Systems** (2nd ed.) | The definitive text on continuous- and discrete-time signals; Fourier, Laplace, Z-transforms. |
| **Harris & Harris — Digital Design and Computer Architecture: RISC-V Edition** | Builds from logic gates to a pipelined RISC-V CPU. The most modern and complete text at this level. |
| **Barr & Massa — Programming Embedded Systems** (2nd ed.) | C in bare-metal environments; useful context even when writing Rust firmware. |

---

## Supplementary
- *The Embedded Rust Book* (online, free) — Rust firmware patterns for ARM Cortex-M
- MIT 6.004 *Computation Structures* (OCW) — RISC-V architecture from gates to OS
- *Making Embedded Systems* — White — practical Rust/C patterns for production firmware
- Patterson & Hennessy — *Computer Organization and Design (RISC-V Edition)* — deeper architecture reference

---

## Rust Simulation Projects

| Project | Description |
|---------|-------------|
| FFT implementation | Cooley-Tukey radix-2 FFT; verified against naive DFT; windowing functions |
| Digital filter designer | FIR windowing and IIR bilinear transform; plot frequency response |
| Bode plotter | Compute gain and phase from a transfer function's poles and zeros |
| RISC-V emulator (RV32I) | Full fetch-decode-execute loop; runs real RISC-V assembly programs |
| UART bit-banging simulator | Model a UART protocol at bit level; test framing error detection |
| Logic gate simulator | Boolean evaluation engine; truth tables; half/full adder, ALU simulation |

The `projects/risc-v-cpu` crate is the main implementation target for the CPU.

---

## Build Projects (Physical Hardware)

| Project | Notes |
|---------|-------|
| Op-amp filter on breadboard | Design, build, and measure a 2nd-order active filter; compare to simulated |
| Microcontroller blinky (bare metal) | RP2040 or STM32 — no HAL, write startup code and GPIO registers from scratch |
| UART driver from scratch | Implement transmit / receive in Rust using only PAC register access |
| ADC + sensor interface | Read an accelerometer or temperature sensor over SPI or I2C |
| H-bridge motor driver | Drive a DC motor with PWM speed control via bare-metal firmware |

---

## Completion Criteria

You are ready for Phase 05 when you can:
1. Design a 2nd-order active low-pass Butterworth filter from a given cutoff frequency
2. Draw the small-signal equivalent of a common-emitter amplifier and compute its gain
3. Implement a DFT from scratch and explain why the FFT is faster
4. Describe the full datapath of a 5-stage pipelined RISC-V CPU
5. Write bare-metal Rust that configures a UART peripheral and transmits a byte without a HAL
