# Phase 14 — VLSI Design

**Duration:** 2–3 months  
**Prerequisites:** Phase 04 (digital design, MOSFET basics), Phase 13 (fabrication — understanding why design rules exist)  
**Goal:** Design a real chip; submit a tapeout through Efabless/OpenMPW

---

## Why This Phase Exists

VLSI design is where you close the loop: you understand the physics, you understand the fab
process, and now you design the actual transistor-level circuits that get manufactured. This
phase covers CMOS cell design (the inverter through complex gates), static timing analysis,
layout, and the full digital design flow from RTL to GDSII. The open-source PDK ecosystem
(SkyWater 130nm, GF 180nm) combined with Google's OpenMPW shuttle programme means you can
submit a real chip for fabrication at zero cost. By the end of this phase you will have either
submitted a real tapeout or completed a complete ASIC physical design.

---

## Learning Objectives

- [ ] Understand CMOS inverter: static and dynamic characteristics, noise margins, power, delay
- [ ] Design static CMOS logic gates: NAND, NOR, complex gates; sizing for equal rise/fall times
- [ ] Understand transmission gates and pass-transistor logic
- [ ] Understand and design standard cells: layout, drive strength variants, well taps, DRC
- [ ] Perform static timing analysis (STA): setup/hold time, propagation delay, clock skew
- [ ] Analyse and minimise dynamic and static power in CMOS circuits
- [ ] Understand SRAM cell design: 6T cell, read/write stability, sense amplifier
- [ ] Complete an RTL-to-GDSII flow using open-source tools (OpenLane / OpenROAD)
- [ ] Understand and use a PDK: design rules, SPICE models, layer definitions, standard cell library
- [ ] Submit a design through the Efabless OpenMPW shuttle (or equivalent)

---

## Topics

### 1. CMOS Inverter — The Fundamental Building Block

#### Static Characteristics
- CMOS inverter circuit: complementary NMOS pull-down + PMOS pull-up
- DC voltage transfer characteristic (VTC): full derivation
  - $V_{OL}$ and $V_{OH}$: output voltage levels
  - $V_{IL}$ and $V_{IH}$: input switching thresholds
  - Noise margins: $NM_H = V_{OH} - V_{IH}$; $NM_L = V_{IL} - V_{OL}$
  - Logic threshold $V_M$: where $V_{out} = V_{in}$; effect of sizing ratio $\beta_p/\beta_n$
- Static power: leakage current (subthreshold, gate, junction); scales with technology node
- Unity-gain points; gain $g = dV_{out}/dV_{in}$ at $V_M$

#### Dynamic Characteristics
- Gate capacitance ($C_{ox} WL$); diffusion capacitance; interconnect capacitance
- Propagation delays $t_{pHL}$ and $t_{pLH}$: derivation from current charging load capacitance
- Switching point current; short-circuit current during switching
- Dynamic power: $P_{dyn} = \alpha C V_{DD}^2 f$ (activity factor $\alpha$, capacitance $C$, supply $V_{DD}$, frequency $f$)
- Power-delay product; energy-delay product; optimum supply voltage
- Sizing for minimum delay: $\beta=1$ is not optimal; path effort method (logical effort)

### 2. Static CMOS Logic Gates

#### Complementary CMOS
- NAND and NOR: pull-down network (NMOS series/parallel) + pull-up network (PMOS mirror)
- De Morgan duality: PDN and PUN are dual networks
- Series connection in NMOS: larger devices needed; size each transistor for equivalent drive
- Complex gates (AOI, OAI, compound): reducing gate count; logical effort analysis
- Body effect in series NMOS stacks: threshold voltage increase; performance impact

#### Logical Effort
- Effort of a gate: $g_i = C_{in}/(C_{ref})$; characterise each gate type
- Path logical effort $G = \prod g_i$; branching effort $B$; electrical effort $H$
- Minimum delay sizing: each stage has equal effort $\hat{f} = (GBH)^{1/N}$; optimal number of stages

#### Transmission Gates and Pass Transistor Logic
- NMOS-only pass transistor: threshold drop problem; limited voltage swing
- Complementary transmission gate: NMOS + PMOS in parallel; full swing; bidirectional
- XOR from transmission gates; multiplexers; CPL (complementary pass transistor logic)
- Trade-offs: reduced transistor count vs. drive strength and delay

### 3. Sequential Logic and Timing

#### Flip-Flops in CMOS
- SR latch from NOR gates; truth table; transparent latch (D-latch with enable)
- Master-slave D-flip-flop: two latches in series; edge-triggered
- Scan flip-flop (DFF with scan input): used for design for test (DFT)

#### Static Timing Analysis
- Setup time $t_{su}$: data must be stable this long before clock edge
- Hold time $t_h$: data must be stable this long after clock edge
- Clock-to-Q delay $t_{cq}$: output valid after clock edge
- Timing constraint: $t_{cq} + t_{logic} + t_{su} \leq T_{clk}$ (setup path)
- Hold time constraint: $t_{cq} + t_{min} \geq t_h + t_{skew}$ (hold path)
- Clock skew: spatial variation in clock arrival; on-chip variation (OCV); clock tree synthesis goal
- Timing slack: positive = timing passed; negative = timing violated (hold or setup)

#### Clock Distribution
- Clock tree synthesis (CTS): balanced H-tree or mesh; buffer insertion; skew minimisation
- Clock gating: enable flip-flop; eliminate dynamic power when blocks idle
- PLLs: clock multiplication and phase locking (overview)

### 4. CMOS Layout and Design Rules

#### Layout Concepts
- Layers in a PDK: diffusion (OD/ACTIVE), poly (gate), N+/P+ select (doping regions), contacts, metal 1–N, via 1–(N-1)
- Layout of an inverter: NMOS and PMOS drawn in N-well and P-substrate; shared power/ground rails
- Design rule categories: minimum width, minimum space, minimum enclosure, extension, exact dimension, no-touch
- Design rule check (DRC): automated verification that layout satisfies all rules
- Layout vs. schematic (LVS): extract netlist from layout; compare to schematic; ensure equivalence
- Antenna rules: charge accumulation on floating gate during plasma etch; diode tie-offs

#### Standard Cells
- Standard cell height: fixed track height (multiples of routing pitch)
- Cell abuttment: abutting cells share power rails (VDD and GND)
- Pin placement: input/output on horizontal routing tracks
- NAND2, NOR2, INV, XNOR, DFF, MUX2 — design and layout for a cell library
- Drive strength variants: INV_x1, INV_x2, INV_x4 — sized transistors, same logic

#### Floorplanning and Placement
- Floorplan: place macros (SRAM, IO pads, PLLs), power rings, power straps
- Place & route (P&R): standard cell placement; timing-driven placement; global + detailed routing
- Congestion analysis; routing resource estimation
- Filler cells; tap cells; well strap placement (DRC for well contact spacing)

#### Power Integrity
- Power grid design: resistance → IR drop ($V = IR$); affects transistor $V_{DS}$
- Decoupling capacitance: on-chip decap cells; suppress $dI/dt$ voltage spikes
- Electromigration (EM): current density limit in metal wires; $J < J_{max}$; affects reliability

### 5. Memory Design

#### SRAM
- 6T SRAM cell: two cross-coupled inverters + 2 access transistors
- Read stability: cell ratio; read disturb; read SNM (static noise margin)
- Write ability: pull-up ratio; sizing constraints
- Sense amplifier: cross-coupled sense amplifier; small differential → full swing
- Row decoder; column multiplexer; precharge; timing diagram

#### DRAM (Overview for Understanding)
- 1T-1C DRAM cell: capacitor stores charge; access transistor
- Destructive read; refresh cycles; DRAM controller overhead
- Why DRAM is denser but slower than SRAM

#### Flash Memory (Overview)
- Floating gate transistor; charge stored in floating gate shifts $V_T$
- Program: Fowler-Nordheim tunnelling or hot-carrier injection
- Erase: bulk F-N tunnelling; endurance limit (wear)
- NOR vs. NAND flash: random access vs. sequential; erase granularity

### 6. Open-Source VLSI Tools and Flow

#### PDK Study: SkyWater 130nm
- Explore the PDK: `sky130_fd_sc_hd` standard cell library; cell LEF/GDS/SPICE
- Technology file: layer definitions, parasitic extraction rules
- Design rules: spacings, widths — understand each one's physical basis

#### RTL-to-GDSII Flow with OpenLane / OpenROAD
1. RTL description (Verilog / SystemVerilog)
2. Synthesis (Yosys): RTL → gate-level netlist using standard cells
3. Floorplanning (OpenROAD): die size, IO placement, power plan
4. Placement (OpenROAD): place standard cells; timing-driven
5. Clock tree synthesis (OpenROAD): buffer insertion, skew minimisation
6. Routing (OpenROAD): global + detailed routing; DRC clean
7. Sign-off: static timing (OpenSTA), LVS (Magic/Klayout), DRC (Magic/Klayout), antenna check
8. GDSII generation: final layout for foundry submission

#### Tapeout via Efabless / OpenMPW
- Submit GDSII + required wrappers through the Efabless platform
- Google-sponsored shuttle runs: free fabrication for qualifying open-source designs
- Caravel harness (management core + user project area): use the standard harness
- Post-fabrication: chip-on-board assembly; probing; debugging

---

## Core Texts

| Text | Notes |
|------|-------|
| **Weste & Harris — CMOS VLSI Design** (4th ed.) | The standard reference. Covers all of this phase. Read and work through Chapters 1–9. |
| **Rabaey, Chandrakasan & Nikolic — Digital Integrated Circuits** (2nd ed.) | Stronger on power and timing analysis. Good complement to Weste & Harris. |
| **Uyemura — Introduction to VLSI Circuits and Systems** | More accessible first reading; good on layout and design flow |

---

## Supplementary
- SkyWater 130nm PDK GitHub (`google/skywater-pdk`) — real PDK; study the layer stack
- OpenLane documentation (efabless/openlane) — open RTL-to-GDSII flow
- Efabless OpenMPW — free tapeout programme (efabless.com/open_shuttle_program)
- Sam Zeloof's PMOS IC series — understanding how small-scale fab connects to design
- MIT 6.374 *Analysis and Design of Digital Integrated Circuits* (OCW)

---

## Rust Simulation Projects

| Project | Description |
|---------|-------------|
| CMOS inverter VTC simulator | MOSFET drain current model; sweep $V_{in}$ and compute $V_{out}$; plot VTC |
| Logic gate delay estimator | RC network model; compute $t_{pHL}$ and $t_{pLH}$ for NAND / NOR with given sizing |
| STA path analyser | Build a graph of logic stages; propagate timing; compute slack for each path |
| Cell sizing optimiser | Minimise delay on a critical path using logical effort; gradient descent on sizing |
| SRAM read/write simulator | 6T cell current equations; simulate read stability and write ability |
| Placement algorithm (toy) | Force-directed or simulated annealing placement for a small netlist |

---

## Completion Criteria

You are done with this phase when you can:
1. Derive the CMOS inverter VTC and explain all four regions with transistor equations
2. Size a NAND2 gate for equal pull-up and pull-down drive strength and describe the methodology
3. Explain setup time, hold time, and clock-to-Q delay; construct the timing constraint equation
4. Describe the full RTL-to-GDSII flow tool-by-tool; know what each tool does
5. Have submitted (or be preparing to submit) a working design to Efabless OpenMPW
