#![feature(iter_array_chunks)]
#![feature(portable_simd)]

use std::simd::num::SimdUint;
use std::simd::u8x32;

const SIMD_CHUNK_SIZE: usize = 32;

const TOLERANCE: f32 = 0.0;
const THRESHOLD: u8 = 5;

fn formula(buffer: Vec<u8>) -> u32 {
    let len: u32 = buffer.len() as u32;
    let sum: u32 = buffer.into_iter().map(u32::from).sum();
    let mean: u8 = (sum / len).try_into().unwrap();
    let mean: f32 = (mean as f32).powf(1.1);

    let diff: u8 = (TOLERANCE * mean) as u8 + THRESHOLD;
    len * diff as u32
}

pub fn find_best(screen: &Image, sample: &Image) -> Option<[u16; 2]> {
    let results = MatchResult::new(screen, sample);

    let best = results.min_by_key(|r| r.0);
    best.map(|b| b.1)
}

pub fn find_best_with_hint(screen: &Image, sample: &Image, coords: [u16; 2]) -> Option<[u16; 2]> {
    if matches_at(screen, sample, coords) {
        Some(coords)
    } else {
        find_best(screen, sample)
    }
}

pub fn find_best_without_threshold(screen: &Image, sample: &Image) -> (u8, [u16; 2]) {
    let result = MatchResult::new_without_threshold(screen, sample)
        .min_by_key(|r| r.0)
        .unwrap();

    let len: u32 = (sample.height * sample.row_len / SIMD_CHUNK_SIZE * SIMD_CHUNK_SIZE).try_into().unwrap();
    ((result.0 / len).try_into().unwrap(), result.1)
}

pub fn find_nth(screen: &Image, sample: &Image, n: usize) -> Option<[u16; 2]> {
    let w: u16 = sample.width.try_into().unwrap();
    let h: u16 = sample.width.try_into().unwrap();

    let data = MatchResult::new(screen, sample);
    let mut filtered: Vec<(u32, [u16; 2])> = Vec::new();

    'outer: for (diff, coords) in data {
        for (min_diff, best_coords) in filtered.iter_mut() {
            if coords[0].abs_diff(best_coords[0]) < w && coords[1].abs_diff(best_coords[1]) < h {
                if diff < *min_diff {
                    *min_diff = diff;
                    *best_coords = coords;
                }
                continue 'outer;
            }
        }

        filtered.push((diff, coords));
        if filtered.len() > n && coords[1] > filtered.last().unwrap().1[1] + h {
            break;
        }
    }

    filtered.get(n).map(|t| t.1)
}

pub fn matches_with_hint(screen: &Image, sample: &Image, coords: [u16; 2]) -> bool {
    matches_at(screen, sample, coords) || matches(screen, sample)
}

pub fn matches(screen: &Image, sample: &Image) -> bool {
    MatchResult::new(screen, sample).next().is_some()
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

    let threshold = formula(sample.buffer.clone());

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


struct MatchResult<'a> {
    screen_rows: Vec<&'a [u8]>,
    sample_rows: Vec<[u8; SIMD_CHUNK_SIZE]>,
    sample_row_len: usize,
    sample_height: usize,
    x_positions: usize,
    y_positions: usize,
    x_position: usize,
    y_position: usize,
    step: usize,
    threshold: u32,
    channels: usize,
}

impl<'a> Iterator for MatchResult<'a> {
    type Item = (u32, [u16; 2]);

    fn next(&mut self) -> Option<Self::Item> {
        let channels = self.channels;
        let threshold = self.threshold;
        let step = self.step;

        let x_positions = self.x_positions;
        let y_positions = self.y_positions;

        while self.y_position <= y_positions {
            while self.x_position <= x_positions {
                let start_y = self.y_position;
                let start_x = self.x_position;
                let end_x = self.x_position + self.sample_row_len;

                let window = self.screen_rows[self.y_position..self.y_position + self.sample_height]
                    .iter()
                    .step_by(step)
                    .flat_map(|&row| row[start_x..end_x].iter().copied().array_chunks());

                let diff_sum = unsafe {
                    match_window(self.sample_rows.iter().copied(), window, threshold)
                };

                self.x_position += channels;
                if diff_sum <= threshold {
                    let x = (start_x / channels).try_into().unwrap();
                    let y = start_y.try_into().unwrap();
                    return Some((diff_sum, [x, y]));
                }
            }
            self.y_position += 1;
            self.x_position = 0;
            // println!("{}", self.y_position);
        }
        None
    }
}

enum Threshold {
    Max,
    Formula,
}

impl<'a> MatchResult<'a> {
    fn new(
        screen: &'a Image,
        sample: &'a Image,
    ) -> Self {
        Self::base(screen, sample, Threshold::Formula)
    }

    fn new_without_threshold(screen: &'a Image, sample: &'a Image) -> Self {
        Self::base(screen, sample, Threshold::Max)
    }

    fn base(
        screen: &'a Image,
        sample: &'a Image,
        threshold: Threshold,
    ) -> Self {
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

        let screen_height = screen.height;
        let sample_height = sample.height;

        let channels = screen.channels;

        let screen_row_len = screen.row_len;
        let sample_row_len = sample.row_len;

        let x_positions = screen_row_len - sample_row_len;
        let y_positions = screen_height - sample_height;

        let step = sample_height.isqrt();

        let screen_rows: Vec<&'a [u8]> = screen.buffer
            .chunks_exact(screen_row_len)
            .collect();
        let sample_rows: Vec<[u8; SIMD_CHUNK_SIZE]> = sample
            .buffer
            .chunks_exact(sample_row_len)
            .step_by(step)
            .flat_map(|row| {
                row.iter()
                    .copied()
                    .array_chunks::<SIMD_CHUNK_SIZE>()
            })
            .collect();

        let threshold = match threshold {
            Threshold::Max => u32::MAX,
            Threshold::Formula => formula(sample_rows.clone().into_flattened())
        };

        Self {
            screen_rows,
            sample_rows,
            sample_row_len,
            sample_height,
            x_positions,
            y_positions,
            x_position: 0,
            y_position: 0,
            step,
            threshold,
            channels,
        }
    }
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
