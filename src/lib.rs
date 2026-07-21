#![feature(iter_array_chunks)]
#![feature(portable_simd)]
#![feature(alloc_slice_into_array)]

use std::simd::num::SimdUint;
use std::simd::u8x32;

const SIMD_CHUNK_SIZE: usize = 32;
type SimdChunk = [u8; SIMD_CHUNK_SIZE];

pub type Point = [u16; 2];
pub type Region = [Point; 2];


pub fn find_best(screen: &Image, sample: &Image, tolerance: u8) -> Option<Point> {
    MatchResult::new(screen, sample, None, tolerance)
        .unwrap()
        .min_by_key(|r| r.0)
        .map(|b| b.1)
}

pub fn get_tolerance(screen: &Image, sample: &Image) -> (u8, Point) {
    MatchResult::new(screen, sample, None, u8::MAX)
        .unwrap()
        .min_by_key(|r| r.0)
        .unwrap()
}

pub fn get_tolerance_in_region(screen: &Image, sample: &Image, region: Region) -> (u8, Point) {
    MatchResult::new(screen, sample, Some(region), u8::MAX)
        .unwrap()
        .min_by_key(|r| r.0)
        .unwrap()
}

pub fn get_nth_tolerance(screen: &Image, sample: &Image, n: usize) -> (u8, Point) {
    let mut results = MatchResult::new(screen, sample, None, u8::MAX)
        .unwrap()
        .filter();
    results.sort();

    results[n]
}

pub fn get_nth_tolerance_in_region(screen: &Image, sample: &Image, region: Region, n: usize) -> (u8, Point) {
    let mut results = MatchResult::new(screen, sample, Some(region), u8::MAX)
        .unwrap()
        .filter();
    results.sort();
    println!("{results:?}");
    results[n]

}

pub fn get_region(screen: &Image, sample: &Image) -> Region {
    let [x, y] = MatchResult::new(screen, sample, None, u8::MAX)
        .unwrap()
        .min_by_key(|r| r.0)
        .unwrap()
        .1;
    let [w, h] = sample.dimensions();
    [[x, y], [x + w, y + h]]
}

pub fn find_in_region(
    screen: &Image,
    sample: &Image,
    region: Region,
    tolerance: u8,
) -> Option<Point> {
    MatchResult::new(screen, sample, Some(region), tolerance).unwrap().min_by_key(|r| r.0).map(|r| r.1)
}

pub fn find_nth(screen: &Image, sample: &Image, tolerance: u8, n: usize) -> Option<Point> {
    MatchResult::new(screen, sample, None, tolerance)
        .ok()
        ?
        .filter()
        .get(n)
        .map(|t| t.1)
}

pub fn find_nth_in_region(screen: &Image, sample: &Image, region: Region, tolerance: u8, n: usize) -> Option<Point> {
    MatchResult::new(screen, sample, Some(region), tolerance).unwrap().filter().get(n).map(|t| t.1)
}

pub fn count(screen: &Image, sample: &Image, tolerance: u8) -> usize {
    MatchResult::new(screen, sample, None, tolerance).unwrap().filter().len()
}

pub fn count_in_region(screen: &Image, sample: &Image, region: Region, tolerance: u8) -> usize {
    MatchResult::new(screen, sample, Some(region), tolerance).unwrap().filter().len()
}

pub fn find_all(screen: &Image, sample: &Image, tolerance: u8) -> Vec<Point> {
    MatchResult::new(screen, sample, None, tolerance).unwrap().filter().into_iter().map(|r| r.1).collect()
}

pub fn find_all_in_region(screen: &Image, sample: &Image, region: Region, tolerance: u8) -> Vec<Point> {
    MatchResult::new(screen, sample, Some(region), tolerance).unwrap().filter().into_iter().map(|r| r.1).collect()
}

pub fn matches(screen: &Image, sample: &Image, tolerance: u8) -> bool {
    MatchResult::new(screen, sample, None, tolerance).unwrap().next().is_some()
}

pub fn matches_at(screen: &Image, sample: &Image, coords: Point, tolerance: u8) -> bool {
    match_at(screen, sample, coords, tolerance)
}

pub fn matches_in_region(screen: &Image, sample: &Image, region: Region, tolerance: u8) -> bool {
    MatchResult::new(screen, sample, Some(region), tolerance).unwrap().next().is_some()
}

pub fn debug_match(screen: &Image, sample: &Image, point: Point) -> SimdChunk {
    let start = usize::from(screen.row_len) * usize::from(point[1]) + usize::from(point[0]) * usize::from(screen.channels);
    u8x32::from_array(
        screen.buffer[start..start + SIMD_CHUNK_SIZE].try_into().unwrap()
    ).abs_diff(
        u8x32::from_array(sample.buffer.first_chunk::<SIMD_CHUNK_SIZE>().unwrap().to_owned())
    ).to_array()
}

fn match_at(screen: &Image, sample: &Image, coords: Point, tolerance: u8) -> bool {
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
        usize::from(sample.row_len) >= SIMD_CHUNK_SIZE,
        "sample width is too small. required at least {}",
        (SIMD_CHUNK_SIZE as f32 / screen.channels as f32).ceil() as u16
    );

    let channels = screen.channels;

    let sample_h = sample.height;
    let sample_row_len = sample.row_len;

    let [start_x, start_y] = coords;

    let start_x = start_x;
    let start_y = start_y;

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

    let start_row = start_x * channels;
    let raw_screen: Vec<SimdChunk> = screen
        .buffer
        .chunks_exact(screen.row_len.into())
        .skip(start_y.into())
        .take(sample_h.into())
        .flat_map(|row| &row[start_row.into()..(start_row + sample_row_len).into()])
        .copied()
        .array_chunks::<SIMD_CHUNK_SIZE>()
        .collect();

    let threshold = u32::from(tolerance) * u32::try_from(sample.buffer.len()).unwrap();

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
    sample_rows: Vec<SimdChunk>,
    sample_row_len: usize,
    sample_height: usize,
    x_positions: usize,
    y_positions: usize,
    x_position: usize,
    y_position: usize,
    step: usize,
    threshold: u32,
    channels: usize,
    pixels: u32,
    x: u16,
    y: u16,
}

impl<'a> Iterator for MatchResult<'a> {
    type Item = (u8, Point);

    fn next(&mut self) -> Option<Self::Item> {
        let channels = self.channels;
        let threshold = self.threshold;
        let step = self.step;

        let x_positions = self.x_positions;
        let y_positions = self.y_positions;

        let sample_h = self.sample_height;
        let sample_row_len = self.sample_row_len;

        while self.y_position <= y_positions {
            while self.x_position <= x_positions {
                let end_x = self.x_position + sample_row_len;

                let window = self.screen_rows[self.y_position..self.y_position + sample_h]
                    .iter()
                    .step_by(step)
                    .copied()
                    .flat_map(|row| row[self.x_position..end_x].iter().copied().array_chunks());

                let diff_sum = unsafe {
                    match_window(self.sample_rows.iter().copied(), window, threshold)
                };

                self.x_position += channels; // add before check
                if diff_sum <= threshold {
                    let x = u16::try_from(self.x_position / channels - 1).unwrap() + self.x; // - 1 because of adding before checking
                    let y = u16::try_from(self.y_position).unwrap() + self.y;
                    let diff = (diff_sum / self.pixels).try_into().unwrap();
                    return Some((diff, [x, y]));
                }
            }
            self.y_position += 1;
            self.x_position = 0;
            // println!("{}", self.y_position);
        }
        None
    }
}

impl<'a> MatchResult<'a> {
    /// Creates a new match iterator.
    ///
    /// # Errors
    ///
    /// Returns an error when the selected region is not fully inside the screen.
    fn new(
        screen: &'a Image,
        sample: &'a Image,
        region: Option<Region>,
        tolerance: u8,
    ) -> Result<Self, String> {
        let screen_width = screen.width;
        let sample_width = sample.width;

        let screen_height = screen.height;
        let sample_height = sample.height;

        let channels = screen.channels;
        let step = sample_height.isqrt().into();

        let screen_row_len = screen.row_len;
        let sample_row_len = sample.row_len;

        let [x0, y0, x1, y1] = *region
            .unwrap_or([[0, 0], screen.dimensions()])
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .into_array()
            .unwrap();

        let [w, h] = [x1 - x0, y1 - y0];
        if w < sample_width || h < sample_height {
            return Err(format!(
                "Region cannot be smaller than sample. region size: {}x{} (from {}x{} to {}x{}), sample_size: {}x{}",
                w,
                h,
                x0,
                y0,
                x1,
                y1,
                sample_width,
                sample_height,
            ))
        }

        if x1 > screen_width || y1 > screen_height {
            return Err(format!(
                "region is not fully inside the screen. screen: {}x{}, region: from {}x{} to {}x{}",
                screen_width,
                screen_height,
                x0,
                y0,
                x1,
                y1,
            ));
        }

        let row_start = (x0 * channels).into();
        let row_end = (x1 * channels).into();

        let x_positions = (w * channels - sample_row_len).into();
        let y_positions = (h - sample_height).into();

        let screen_rows: Vec<&'a [u8]> = screen.buffer
            .chunks_exact(screen_row_len.into())
            .take(y1.into())
            .skip(y0.into())
            .map(|row| &row[row_start..row_end])
            .collect();
        let sample_rows: Vec<SimdChunk> = sample
            .buffer
            .chunks_exact(sample_row_len.into())
            .step_by(step)
            .flat_map(|row| {
                row.iter()
                    .copied()
                    .array_chunks::<SIMD_CHUNK_SIZE>()
            })
            .collect();
        let pixels = u32::try_from(sample_rows.len() * SIMD_CHUNK_SIZE).unwrap();
        let threshold = u32::from(tolerance) * pixels;
        
        Ok(Self {
            screen_rows,
            sample_rows,
            sample_row_len: sample_row_len.into(),
            sample_height: sample_height.into(),
            x_positions,
            y_positions,
            x_position: 0,
            y_position: 0,
            step,
            threshold,
            channels: channels.into(),
            pixels,
            x: x0.into(),
            y: y0.into(),
        })
    }

    fn filter(self) -> Vec<(u8, Point)> {
        let w = (self.sample_row_len / self.channels).try_into().unwrap();
        let h = self.sample_height.try_into().unwrap();

        let mut filtered: Vec<(u8, Point)> = Vec::new();
        'outer: for (diff, coords) in self {
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
        }
        filtered
    }
}


#[target_feature(enable = "avx2")]
unsafe fn match_window(
    win1: impl Iterator<Item = SimdChunk>,
    win2: impl Iterator<Item = SimdChunk>,
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
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) channels: u16,
    pub(crate) row_len: u16,
}
impl Image {
    pub fn new(
        buffer: Vec<u8>,
        width: u16,
        height: u16,
        channels: u16,
    ) -> Result<Image, String> {
        let prod: u32 = u32::from(width) * u32::from(height) * u32::from(channels);
        if prod != buffer.len().try_into().unwrap() {
            return Err(format!(
                "Buffer size mismatch: expected {}×{}×{} = {} bytes, got {} bytes",
                width,
                height,
                channels,
                prod,
                buffer.len()
            ));
        }

        Ok(Self {
            buffer,
            width,
            height,
            channels,
            row_len: width * u16::from(channels),
        })
    }

    pub fn width(&self) -> u16 {
        self.width
    }
    pub fn height(&self) -> u16 {
        self.height
    }
    pub fn channels(&self) -> u16 {
        self.channels
    }
    pub fn dimensions(&self) -> Point {
        [self.width, self.height]
    }

    pub fn as_raw(&self) -> &Vec<u8> {
        &self.buffer
    }
}
