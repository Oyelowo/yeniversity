# Phase 12 — Optics & Imaging

**Duration:** 2–3 months  
**Prerequisites:** Phase 02 (electromagnetism, waves), Phase 01 (Fourier transforms)  
**Can start:** After Phase 02

---

## Why This Phase Exists

Light is the fastest, most information-dense medium we can use for sensing, communication, and
measurement. Every camera, lidar, spectrometer, optical fibre, laser, night-vision sensor,
and telescope works on principles covered in this phase. Understanding optics from first
principles — from Maxwell's equations to the modulation transfer function of a lens — is what
allows you to design imaging systems, compute diffraction limits, and build custom sensors for
robotics, aerospace, and scientific instruments.

---

## Learning Objectives

- [ ] Apply the ray optics approximation correctly; trace rays through lens systems; design imaging geometries
- [ ] Understand wave optics: interference, diffraction, coherence; Huygens-Fresnel principle
- [ ] Derive the Fraunhofer diffraction pattern for apertures; understand the PSF and Airy disk
- [ ] Understand and apply the Fourier treatment of optics: spatial frequency, optical transfer function (OTF), modulation transfer function (MTF)
- [ ] Understand laser principles: stimulated emission, population inversion, resonator modes
- [ ] Understand CCD and CMOS image sensor physics; noise sources; signal-to-noise ratio
- [ ] Design a camera system: lens selection, pixel size, diffraction limit, depth of field, noise budget
- [ ] Understand fibre optics and optical waveguides principles

---

## Topics

### 1. Geometric (Ray) Optics

- Fermat's principle of least time; derivation of Snell's law
- Reflection; specular and diffuse; law of reflection
- Refraction; Snell's law $n_1 \sin\theta_1 = n_2 \sin\theta_2$; dispersion; refractive index
- Total internal reflection; critical angle; evanescent waves
- Paraxial approximation; thin lens equation $1/f = 1/d_o + 1/d_i$
- Lens maker's equation; thick lenses; principal planes; principal rays
- Magnification; virtual vs. real images
- Optical instruments: microscope, telescope, camera — design equations
- Aberrations: spherical, coma, astigmatism, field curvature, distortion; chromatic aberration
- Aperture and F-number; depth of field; relationship to diffraction limit
- Ray transfer matrix (ABCD matrices) for cascaded optical elements

### 2. Wave Optics & Interference

#### Electromagnetic Waves and Polarisation
- Review: plane wave solutions to Maxwell's equations; $E$, $B$, $k$, $\omega$ relationships
- Polarisation states: linear, circular, elliptical; Stokes parameters
- Fresnel equations for reflection and transmission at an interface
- Brewster's angle; polarisation by reflection
- Anti-reflection coatings; thin-film interference (constructive and destructive conditions)

#### Two-Beam Interference
- Coherence: temporal and spatial coherence; coherence length; coherence time
- Young's double slit: fringe spacing; visibility; derivation from path difference
- Multi-beam interference: Fabry-Pérot etalon; finesse; free spectral range; applications
- Mach-Zehnder and Michelson interferometers; fringe patterns

### 3. Diffraction

#### Huygens-Fresnel Principle
- Every point on a wavefront is a secondary source; Huygens construction
- Fresnel (near-field) and Fraunhofer (far-field) diffraction regimes; Fresnel number $N_F = a^2 / \lambda z$

#### Fraunhofer Diffraction (Far Field)
- Single slit: intensity $I(\theta) = I_0 (\sin\alpha/\alpha)^2$ where $\alpha = \pi a \sin\theta / \lambda$
- Circular aperture: Airy disk; first zero at $\sin\theta = 1.22\lambda/D$
- Rayleigh criterion for angular resolution: $\Delta\theta_{min} = 1.22\lambda/D$
- Diffraction grating: grating equation; resolving power $R = mN$

#### Point Spread Function and Optical Resolution
- PSF of a diffraction-limited lens: Airy pattern
- Convolution model of image formation: $i(x,y) = o(x,y) * h(x,y)$
- Incoherent imaging: intensity PSF (IPSF); coherent imaging: amplitude PSF (APSF)

### 4. Fourier Optics

- Fraunhofer diffraction as a Fourier transform: $U(f_x, f_y) = \mathcal{F}\{u(x,y)\}$
- Spatial frequency $f_x = x'/\lambda z$; relationship to diffraction angle
- Lens as a Fourier transform operator: object at front focal plane → Fourier transform at back focal plane
- 4-F optical system: two lenses; Fourier filtering in the pupil plane
- Optical transfer function (OTF): Fourier transform of PSF
- Modulation transfer function (MTF): $|OTF|$; meaning for image quality; diffraction-limited MTF
- Phase transfer function (PTF)
- Sampling theorem in imaging: pixel Nyquist frequency; aliasing; anti-aliasing filter role of the optics
- Wavefront aberrations in terms of Zernike polynomials; effect on PSF and MTF

### 5. Lasers

#### Stimulated Emission and Population Inversion
- Blackbody radiation; Einstein A and B coefficients; relationship between absorption, stimulated emission, and spontaneous emission
- Population inversion: why a two-level system cannot lase; three- and four-level schemes
- Pumping mechanisms: optical, electrical discharge, chemical

#### Laser Resonators and Modes
- Fabry-Pérot resonator; longitudinal modes
- Gaussian beam: beam waist $w_0$; Rayleigh range $z_R$; divergence; $M^2$ factor
- Transverse modes (TEM$_{mn}$); dominant TEM$_{00}$
- Ray transfer matrix (ABCD) applied to Gaussian beams: beam propagation and transformation

#### Laser Types and Characteristics
- Gas lasers: He-Ne (632 nm), CO2 (10.6 μm); used in sensing, cutting, metrology
- Solid-state: Nd:YAG; Ti:Sapphire (tunable, ultrafast)
- Diode lasers: semiconductor p-n junction; stripe geometry; high efficiency; telecom LDs
- Fibre lasers; DFB lasers; VCSEL arrays (lidar)
- Pulsed lasers: Q-switching; mode-locking; pulse duration vs. bandwidth (time-bandwidth product)

### 6. Image Sensors

#### Photodetection Physics
- Photoelectric effect; photon energy $E = h\nu$; quantum efficiency $\eta$
- Photoconductor; photodiode (p-n and p-i-n); avalanche photodiode (APD)
- Dark current; shot noise; thermal (Johnson) noise; 1/f noise
- Signal-to-noise ratio (SNR); noise equivalent power (NEP); detectivity $D^*$

#### CCD Sensors
- Charge generation by photons in silicon; depletion region
- Charge collection in potential wells; charge transfer (shift register principle)
- Serial and parallel registers; output node; charge-to-voltage conversion
- Full-well capacity; dynamic range; anti-blooming
- Interline vs. full-frame vs. frame-transfer CCD architectures

#### CMOS Image Sensors
- Active pixel sensor (APS): 3T and 4T pixel circuits; per-pixel amplifier
- Rolling shutter vs. global shutter; readout speed advantages over CCD
- Back-side illumination (BSI); stacked sensors
- On-chip ADC; digital pixel sensors; event cameras

#### Noise Budget for an Imaging System
- Shot noise: $\sigma_{shot} = \sqrt{N}$ (photon counting statistics)
- Read noise; dark current shot noise; fixed pattern noise
- Full SNR model: $SNR = N / \sqrt{N + N_{dark} + \sigma_{read}^2}$
- Exposure triangle: aperture, exposure time, ISO (gain); spatial resolution tradeoff

#### Camera System Design
- Pixel size and lens selection for a given field of view and resolution
- Diffraction limit of a lens: $d_{diff} = 2.44\lambda (f/\#)$; matching to pixel pitch
- Lens MTF and combined system MTF
- Sensor format; crop factor; equivalences

### 7. Fibre Optics & Waveguides

- Total internal reflection as the basis of fibre guidance
- Ray model: acceptance cone; numerical aperture (NA)
- Wave model: modes in a waveguide; cut-off conditions; number of modes
- Single-mode vs. multimode fibres: differences and use cases
- Fibre loss mechanisms: Rayleigh scattering, absorption; attenuation in dB/km
- Dispersion: modal dispersion (MM), chromatic dispersion (SM); effect on bandwidth
- Fibre amplifiers (EDFA); fibre sensors (strain, temperature); fibre gyroscope (Sagnac effect)

---

## Core Texts

| Text | Notes |
|------|-------|
| **Hecht — Optics** (5th ed.) | The comprehensive reference text. Covers geometric optics through wave optics, diffraction, polarisation in depth. |
| **Goodman — Introduction to Fourier Optics** (4th ed.) | The definitive Fourier optics reference. Chapters 2–6 are essential for understanding imaging system analysis. |
| **Saleh & Teich — Fundamentals of Photonics** (3rd ed.) | Covers wave optics, guided waves, lasers, and detectors rigorously. Strong on laser theory and fibre optics. |
| **Nakamura (ed.) — Image Sensors and Signal Processing for Digital Still Cameras** | Detailed treatment of CCD/CMOS physics, readout, noise, and signal processing pipeline. |

---

## Supplementary
- Born & Wolf — *Principles of Optics* — the classical advanced reference; rigorous electromagnetic treatment
- Smith — *Modern Optical Engineering* — practical design guide for optical instruments
- MIT 2.71 *Optics* (OCW) — excellent problem sets; covers Fourier optics well

---

## Rust Simulation Projects

| Project | Description |
|---------|-------------|
| Ray tracer (pinhole + thin lens) | Trace rays through lens systems; compute image position, magnification |
| Diffraction pattern calculator | Single slit, double slit, circular aperture; plot Fraunhofer intensity |
| Fourier optics simulator | 2D FFT-based imaging system; apply pupil mask; compute PSF and MTF |
| Gaussian beam propagation | Track $w(z)$, $R(z)$, Gouy phase through ABCD elements |
| Fabry-Pérot cavity | Transmission vs. wavelength for given $R$ and $d$; finesse |
| Image sensor noise model | Monte Carlo simulation of a pixel's SNR vs. flux and integration time |
| MTF tester simulation | Slanted-edge method to compute MTF from a simulated image of an edge |

---

## Completion Criteria

You are ready to move on when you can:
1. Compute the Airy disk diameter for a lens of given F-number and wavelength
2. Explain the OTF and MTF and what they tell you about image quality
3. Design a camera to achieve a given spatial resolution at a given working distance
4. Sketch the 4-F Fourier filtering optical system and explain what each element does
5. Compute the SNR of a pixel for given quantum efficiency, flux, exposure time, and read noise
