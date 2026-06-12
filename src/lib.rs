#![feature(iter_array_chunks)]
#![feature(portable_simd)]

use std::cmp::min_by_key;
use std::simd::num::SimdUint;
use std::simd::u8x32;

const SIMD_CHUNK_SIZE: usize = 32;

const TOLERANCE: f32 = 0.04;
const THRESHOLD: u8 = 5;

fn formula(buffer: &Vec<u8>) -> u32 {
    let len: u32 = buffer.len() as u32;
    let sum: u32 = buffer.iter().copied().map(u32::from).sum();
    let mean: u8 = (sum / len).try_into().unwrap();

    let diff: u8 = (TOLERANCE * mean as f32) as u8 + THRESHOLD;
    len * diff as u32
}

pub fn find_best(screen: &Image, sample: &Image) -> Option<[u16; 2]> {
    let results: Vec<(u32, [u16; 2])> = match_template(screen, sample);

    if results.is_empty() {
        None
    } else {
        let mut best_result: (u32, [u16; 2]) = (u32::MAX, [0, 0]);
        for result in results {
            best_result = min_by_key(best_result, result, |&(diff, _)| diff)
        }
        Some(best_result.1)
    }
}

pub fn find_best_with_hint(screen: &Image, sample: &Image, coords: [u16; 2]) -> Option<[u16; 2]> {
    if matches_at(screen, sample, coords) {
        Some(coords)
    } else {
        find_best(screen, sample)
    }
}

pub fn find_nth(screen: &Image, sample: &Image, n: usize) -> Option<[u16; 2]> {
    let w: u16 = sample.width.try_into().unwrap();
    let h: u16 = sample.width.try_into().unwrap();

    let data = match_template(screen, sample);
    let coords: Vec<_> = data.into_iter().map(|(_, coords)| coords).collect();
    let mut filtered: Vec<[u16; 2]> = Vec::new();

    'outer: for coord in coords {
        for existing in &filtered {
            if coord[0].abs_diff(existing[0]) < w && coord[1].abs_diff(existing[1]) < h {
                continue 'outer;
            }
        }

        filtered.push(coord);
        if filtered.len() > n {
            break;
        }
    }

    filtered.get(n).copied()
}

pub fn matches_with_hint(screen: &Image, sample: &Image, coords: [u16; 2]) -> bool {
    matches_at(screen, sample, coords) || matches(screen, sample)
}

pub fn matches(screen: &Image, sample: &Image) -> bool {
    !match_template(screen, sample).is_empty()
}

pub fn matches_at(screen: &Image, sample: &Image, coords: [u16; 2]) -> bool {
    assert_eq!(
        screen.channels, sample.channels,
        "screen channels = {}, sample channels = {}",
        screen.channels, sample.channels
    );
    assert!(
        screen.width >= sample.width,
        "screen width = {}, sample width = {}",
        screen.width,
        sample.width
    );
    assert!(
        screen.height >= sample.height,
        "screen height = {}, sample height = {}",
        screen.height,
        sample.height
    );
    assert!(
        sample.row_len >= SIMD_CHUNK_SIZE,
        "sample width is too small. required at least {}",
        (SIMD_CHUNK_SIZE as f32 / screen.channels as f32).ceil() as u16
    );

    let channels = screen.channels;

    let sample_h = sample.height;
    let sample_row_len = sample.row_len;

    let [start_x, start_y] = coords;

    let start_x: usize = start_x.into();
    let start_y: usize = start_y.into();

    assert!(
        start_x <= screen.width - sample.width,
        "start_x = {}, screen_width = {}, sample_width = {}. start_x is too big.",
        start_x,
        screen.width,
        sample.width
    );
    assert!(
        start_y <= screen.height - sample.height,
        "start_y = {}, screen_height = {}. sample_height = {}, start_y is too_big.",
        start_y,
        screen.height,
        sample.height
    );

    let threshold = formula(&sample.buffer);

    let start_row = start_x * channels;
    let raw_screen: Vec<[u8; SIMD_CHUNK_SIZE]> = screen
        .buffer
        .chunks_exact(screen.row_len)
        .skip(start_y)
        .take(sample_h)
        .flat_map(|row| &row[start_row..start_row + sample_row_len])
        .copied()
        .array_chunks::<SIMD_CHUNK_SIZE>()
        .collect();

    let diff_sum = unsafe {
        match_window(
            raw_screen.into_iter(),
            sample.buffer
                .clone()
                .into_iter()
                .array_chunks::<SIMD_CHUNK_SIZE>(),
            threshold,
        )
    };
    diff_sum <= threshold
}

fn match_template(screen: &Image, sample: &Image) -> Vec<(u32, [u16; 2])> {
    assert_eq!(
        screen.channels, sample.channels,
        "different number of channels. screen: {}, sample: {}",
        screen.channels, sample.channels
    );
    assert!(
        sample.row_len >= SIMD_CHUNK_SIZE,
        "sample width is too small. required at least {}",
        (SIMD_CHUNK_SIZE as f32 / screen.channels as f32).ceil() as u16
    );

    let screen_h = screen.height;
    let sample_h = sample.height;

    let channels = screen.channels;

    let screen_row_len = screen.row_len;
    let sample_row_len = sample.row_len;

    let x_positions = screen_row_len - sample_row_len;
    let y_positions = (screen_h - sample_h) * screen_row_len;

    let h_step = sample_h.isqrt();

    let area = sample_h * screen_row_len;

    let screen_buf = &screen.buffer;
    let sample_buf: Vec<[u8; SIMD_CHUNK_SIZE]> = sample
        .buffer
        .chunks_exact(sample_row_len)
        .step_by(h_step)
        .flat_map(|row| {
            row.iter()
                .copied()
                .array_chunks::<SIMD_CHUNK_SIZE>()
        })
        .collect();

    let threshold: u32 = formula(&sample_buf.clone().into_iter().flatten().collect());
    println!("{}", threshold);

    let mut m: u32 = u32::MAX;
    let mut c: [u16; 2] = [0, 0];
    let mut matches: Vec<(u32, [u16; 2])> = Vec::new();
    for pos_y in (0..=y_positions).step_by(screen_row_len) {
        for pos_x in (0..=x_positions).step_by(channels) {
            let rows = screen_buf[pos_y..pos_y + area].chunks_exact(screen_row_len);
            let window = rows.step_by(h_step).flat_map(|row| {
                row[pos_x..pos_x + sample_row_len]
                    .iter()
                    .copied()
                    .array_chunks::<SIMD_CHUNK_SIZE>()
            });

            // if  == 715 &&  == 565 {
            //     return vec![(0, [67, 67])];
            // }

            let diff_sum = unsafe {
                match_window(sample_buf.clone().into_iter(), window, threshold)
            };
            if diff_sum < m {
                m = diff_sum;
                c = [(pos_y / screen_row_len) as u16, (pos_x / channels) as u16];
            }
            if diff_sum <= threshold {
                let x = (pos_x / channels).try_into().unwrap();
                let y = (pos_y / screen_row_len).try_into().unwrap();
                matches.push((diff_sum, [x, y]));
            }
        }
    }
    let rows = screen_buf[c.1 * screen_row_len..c.1 * screen_row_len + area].chunks_exact(screen_row_len);
    let window = rows.step_by(h_step).flat_map(|row| {
    row[c.0 * channels..c.0 * channels + sample_row_len]
        .iter()
        .copied()
        .array_chunks::<SIMD_CHUNK_SIZE>()
    });
    let diff_sum = unsafe {
        match_window(sample_buf.clone().into_iter(), window, 0)
    };
    let l: u32 = (sample_buf.len() * 32) as u32;
    println!("expected: {}, got: {}", threshold / l, diff_sum / l);
    matches
}

#[target_feature(enable = "avx2")]
unsafe fn match_window(
    win1: impl Iterator<Item = [u8; SIMD_CHUNK_SIZE]>,
    win2: impl Iterator<Item = [u8; SIMD_CHUNK_SIZE]>,
    threshold: u32,
) -> u32 {
    let mut diff_sum = 0;

    for (chunk1, chunk2) in win1.zip(win2) {
        diff_sum += u8x32::from_array(chunk1)
            .abs_diff(u8x32::from_array(chunk2))
            .cast::<u32>()
            .reduce_sum();
        if diff_sum > threshold {
            return diff_sum;
        }
    }

    // for (a, b) in chunks1.into_remainder().zip(chunks2.into_remainder()) {
    //     diff_sum += u32::from(a.abs_diff(b));
    // }
    diff_sum
}

pub struct Image {
    pub(crate) buffer: Vec<u8>,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) channels: usize,
    pub(crate) row_len: usize,
}
impl Image {
    pub fn new(
        buffer: Vec<u8>,
        width: usize,
        height: usize,
        channels: usize,
    ) -> Result<Image, String> {
        if width * height * channels != buffer.len() {
            return Err(format!(
                "Buffer size mismatch: expected {}×{}×{} = {} bytes, got {} bytes",
                width,
                height,
                channels,
                width * height * channels,
                buffer.len()
            ));
        }

        Ok(Self {
            buffer,
            width,
            height,
            channels,
            row_len: width * channels,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn channels(&self) -> usize {
        self.channels
    }

    pub fn as_raw(&self) -> &Vec<u8> {
        &self.buffer
    }
}
