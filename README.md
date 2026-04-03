# Rust-Donut: Fixed-Point 3D Rendering

A high-performance Rust implementation of the classic `donut.c` 3D ASCII animation. This project ports the original obfuscated C logic into a memory-safe, idiomatic Rust environment using fixed-point arithmetic and CORDIC-like rotation approximations.

---

## Overview

This engine renders a rotating 3D torus in the terminal using only ASCII characters. Unlike standard graphics pipelines, it avoids floating-point math ($f32/f64$) entirely, relying on **bit-shifting and integer math** to calculate projections and lighting.

### Key Engineering Features
* **Zero FPU Usage:** All trigonometric rotations are approximated via integer bit-shifts.
* **Z-Buffering:** Implements a depth buffer to handle occlusions and 3D geometry consistency.
* **Memory Safety:** Eliminates the Undefined Behavior (UB) present in the original C source by implementing strict bounds checking on the luminance map.
* **Perspective Projection:** Maps 3D $(x, y, z)$ coordinates to a 2D $(x', y')$ terminal grid.

---

## Technical Implementation

### The Rotation Logic
The core "macro" from the original C code is translated into an idiomatic Rust function (or macro) that performs vector rotation using the following logic:

$$x_{next} = x - \frac{mul \cdot y}{2^{shift}}$$
$$y_{next} = y + \frac{mul \cdot x_{orig}}{2^{shift}}$$

### Rust vs. C: The Safety Delta

| Feature | Original C | Rust Port |
| :--- | :--- | :--- |
| **Indexing** | Unchecked (Potential Segfault) | Checked (Clamped/Safe) |
| **Arithmetic** | Implementation-Defined Shifts | Strict Arithmetic Shifts |
| **I/O** | `printf` (Unbuffered) | `std::io::stdout` (Lock-sync) |

---

## Usage

### Prerequisites
* [Rust Toolchain (Cargo)](https://rustup.rs/)
* A terminal supporting **ANSI Escape Sequences**

### Execution
```bash
# Clone the repository
git clone [https://github.com/YOUR_USERNAME/rust-donut.git](https://github.com/YOUR_USERNAME/rust-donut.git)
cd rust-donut

# Run with optimizations (Required for smooth animation)
cargo run --release
```

### Critical Analysis & Risks
* Integer Drift (High Confidence): Because the rotation uses integer approximations rather than exact sin/cos values, the torus may experience slight "shape decay" or "wobble" over extremely long execution periods as errors accumulate.
* Terminal Resize (Risk): The rendering logic assumes a fixed-width buffer (80 characters). Resizing the terminal window during execution will likely corrupt the frame alignment.
* ANSI Dependency: The "frame reset" relies on the escape sequence \x1b[23A. Non-compliant terminals will output a vertical stream of frames rather than an animation.
* Alternative Perspective: While Rust provides safety, it increases the verbosity of the code. The original C version is a "code golf" exercise in brevity; this Rust version is an exercise in predictable systems engineering.
