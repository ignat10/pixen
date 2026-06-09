const TOLERANCE: f32 = 0.02;
const THRESHOLD: u8 = 5;

fn formula(buffer: &Vec<u8>) -> u32 {
    let len: u32 = buffer.len() as u32;
    let sum: u32 = buffer.iter().copied().map(u32::from).sum();
    let mean: u8 = (sum / len).try_into().unwrap();
    
    let diff: u8 = (TOLERANCE * mean as f32) as u8 + THRESHOLD;
    len * diff as u32
}


pub fn images_match(
    screen: &Image,
    sample: &Image,
    coords: [u16; 2]
) -> bool {
    let channels = screen.channels;

    let sample_h = sample.height;
    let sample_row_len = sample.row_len;

    let [start_x, start_y] = coords;

    let start_x: usize = start_x.into();
    let start_y: usize = start_y.into();

    assert_eq!(screen.channels, sample.channels, "screen channels = {}, sample channels = {}", screen.channels, sample.channels);
    assert!(screen.width >= sample.width, "screen width = {}, sample width = {}", screen.width, sample.width);
    assert!(screen.height >= sample.height, "screen height = {}, sample height = {}", screen.height, sample.height);

    assert!(start_x <= screen.width - sample.width, "start_x = {}, screen_width = {}, sample_width = {}. start_x is too big.", start_x, screen.width, sample.width);
    assert!(start_y <= screen.height - sample.height, "start_y = {}, screen_height = {}. sample_height = {}, start_y is too_big.", start_y, screen.height, sample.height);

    let threshold = formula(&sample.buffer);
    
    let start_row = start_x * channels;
    let raw_screen = screen.buffer
        .chunks_exact(screen.row_len)
        .skip(start_y)
        .take(sample_h)
        .flat_map(|row| &row[start_row..start_row + sample_row_len])
        .copied()
        .collect::<Vec<u8>>();

    let diff_sum = match_window(
        &raw_screen,
        &sample.buffer,
        0,
        sample_row_len,
        channels,
        sample_row_len,
        sample_row_len * sample_h,
        threshold
    );
    diff_sum <= threshold
}

pub fn find_first(screen: &Image, sample: &Image) -> Option<[u16; 2]> {
    Some(
        match_template(screen, sample)
            .next()?
            .1
    )
}


pub fn find_best(screen: &Image, sample: &Image) -> Option<[u16; 2]> {
    let results: Vec<(u32, [u16; 2])> = match_template(screen, sample).collect();

    let mut best_diff = u32::MAX;
    let mut best_coords: Option<[u16; 2]> = None;
    for (diff, coords) in results {
        if diff < best_diff {
            best_coords = Some(coords);
            best_diff = diff;
        }
    }
    best_coords
}


pub fn find_nth(
    screen: &Image,
    sample: &Image,
    n: usize
) -> Option<[u16; 2]> {
    let w: u16 = sample.width.try_into().unwrap();
    let h: u16 = sample.width.try_into().unwrap();

    let data = match_template(screen, sample);
    let coords: Vec<_> = data.map(|(_, coords)| coords).collect();
    let mut filtered: Vec<[u16; 2]> = Vec::new();

    'outer: for coord in coords {
        for existing in &filtered {
            if coord[0].abs_diff(existing[0]) < w
                && coord[1].abs_diff(existing[1]) < h
            {
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


fn match_template(
    screen: &Image,
    sample: &Image,
) -> impl Iterator<Item=(u32, [u16; 2])> {
    assert_eq!(screen.channels, sample.channels, "different number of channels. screen: {}, sample: {}", screen.channels, sample.channels);
    let screen_h = screen.height;
    let sample_h = sample.height;

    let channels = screen.channels;

    let screen_row_len = screen.row_len;
    let sample_row_len = sample.row_len;

    let x_positions = screen_row_len - sample_row_len;
    let y_positions = (screen_h - sample_h) * screen_row_len;

    let w_step = sample.width.isqrt() * channels;
    let h_step = sample_h.isqrt();

    let area = sample_h * screen_row_len;

    let sample_sum: u32 = sample.buffer.iter().copied().map(u32::from).sum();
    let sample_mean: u8 = (sample_sum as usize / sample.buffer.len()).try_into().unwrap();

    let screen_buf = &screen.buffer;
    let sample_buf: Vec<u8> = sample.buffer
        .chunks_exact(sample_row_len)
        .step_by(h_step)
        .flat_map(|row|
                (0..sample_row_len)
                    .step_by(w_step)
                    .flat_map(|idx|
                        row[idx..idx + channels]
                            .iter()
                            .copied()
                    )
                    .collect::<Vec<u8>>()
            )
        .collect();

    let threshold: u32 =  formula(&sample_buf);

    (0..=y_positions).step_by(screen_row_len).flat_map(move |pos_y| {
        (pos_y..=pos_y + x_positions).step_by(channels).filter_map(|pos_x| {

            let diff_sum = match_window(
                screen_buf,
                &sample_buf,
                pos_x,
                sample_row_len,
                w_step,
                h_step * screen_row_len,
                area,
                threshold
            );

            // println!("{}", diff_sum as f32 / (sample_buf.len() * u8::MAX as usize) as f32);  // tolerance needed for the best match
            if diff_sum <= threshold {
                let x = (pos_x % screen_row_len / channels).try_into().unwrap();
                let y = (pos_x / screen_row_len).try_into().unwrap();
                Some((diff_sum, [x, y]))
            } else {
                None
            }
        }).collect::<Vec<(u32, [u16; 2])>>()
    })
}

#[inline(always)]
fn match_window(
    screen: &Vec<u8>,
    sample: &Vec<u8>,
    screen_row: usize,
    sample_row_len: usize,
    w_step: usize,
    h_step: usize,
    area: usize,
    min_diff: u32,
) -> u32 {
    let mut diff_sum = 0;

    let mut channel = 0;

    let mut row = screen_row;
    let max_row = screen_row + area;
    'a: while row < max_row {
        let mut idx = row;
        let max_idx = row + sample_row_len;
        while idx < max_idx {
            for (&scr, &smp) in screen[idx..idx + 3].iter().zip(sample[channel..].iter()) {
                diff_sum += u32::from(scr.abs_diff(smp));
                channel += 1;
            }
            if diff_sum > min_diff {
                break 'a;
            }
            idx += w_step;
        }
        row += h_step;
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

        Ok(
            Self {
                buffer,
                width,
                height,
                channels,
                row_len: width * channels,
            }
        )
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    pub fn channels(&self) -> usize { self.channels }

    pub fn as_raw(&self) -> &Vec<u8> { &self.buffer }
}
