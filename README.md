# pixen

High-performance Rust image matching and pixel search library using SIMD acceleration.

## Overview

pixen is a low-level image matching engine designed for fast template detection inside raw pixel buffers.

It focuses on:
- fast approximate image matching
- SIMD-accelerated pixel comparison (AVX2)
- configurable tolerance-based matching
- screen → template search operations

Typical use cases:
- screen automation / botting engines
- UI element detection
- game screen analysis
- computer vision prototypes
- performance-critical pixel search

This is not a general image processing library. It is optimized for one thing: finding images inside images fast.

## Core Concept

The library compares a sample image against a larger screen image by sliding a window and computing pixel differences using SIMD (`u8x32` chunks).

A match is valid if:

```
sum(abs_diff) <= tolerance * number_of_pixels
```

## Features

- SIMD-accelerated comparison (AVX2 target feature)
- Sliding-window template matching
- Optional hinted fast-path search
- Nth match lookup
- Match counting
- Early-exit optimization on threshold breach
- Per-pixel tolerance control

## Installation

```toml
[dependencies]
pixen = { git = "https://github.com/ignat10/pixen" }
```

## Image Format

Images use raw buffers:

- Row-major layout
- Each pixel = `channels` bytes (usually RGBA)
- No decoding/encoding included

```rust
pub struct Image {
    pub(crate) buffer: Vec<u8>,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) channels: usize,
}
```

## Basic Example

```rust
use pixen::{Image, find_best};

fn main() {
    let screen = Image::new(screen_buffer, 1920, 1080, 4).unwrap();
    let template = Image::new(icon_buffer, 32, 32, 4).unwrap();

    if let Some([x, y]) = find_best(&screen, &template, 10) {
        println!("Found at: {x}, {y}");
    }
}
```

## API Overview

### Finding matches

- find_best(screen, sample, tolerance)
- find_best_with_hint(screen, sample, coords, tolerance)
- find_nth(screen, sample, tolerance, n)
- count(screen, sample, tolerance)

### Checking matches

- matches(screen, sample, tolerance)
- matches_at(screen, sample, coords, tolerance)
- matches_with_hint(screen, sample, coords, tolerance)

### Utility

- get_tolerance(screen, sample)

## Performance Notes

- Uses AVX2 SIMD (`std::simd::u8x32`)
- 32-byte chunk processing
- Early exit when threshold exceeded
- Step-based scan optimization
- Optimized for CPU throughput

## Constraints

- Requires nightly Rust (`portable_simd`)
- AVX2 recommended for performance
- Works on raw pixel buffers only
