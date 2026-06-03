const TOLERANCE: f32 = 0.1;



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

    let start_row = start_x * channels;

    let raw_screen = screen.buffer
        .chunks_exact(screen.row_len)
        .skip(start_y)
        .take(sample_h)
        .flat_map(|row| &row[start_row..start_row + sample_row_len])
        .copied()
        .map(i16::from)
        .collect::<Vec<i16>>();

    let raw_sample: Vec<i16> = sample.buffer
        .iter()
        .map(|&a| i16::from(a))
        .collect();


    let diff_sum = match_window(
        &raw_screen,
        &raw_sample,
        0,
        sample_row_len,
        channels,
        sample_row_len,
        sample_row_len * sample_h,
        0,
        channels,
        (TOLERANCE * (raw_sample.len() * u8::MAX as usize) as f32) as u32
    );
    TOLERANCE > diff_sum as f32 / (raw_sample.len() * u8::MAX as usize) as f32
}


pub fn find_sample(
    screen: &Image,
    sample: &Image,
) -> Option<[u16; 2]> {
    assert_eq!(screen.channels, sample.channels);
    let screen_buf = &screen.buffer;
    let sample_buf = &sample.buffer;

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

    let screen_int_buf: Vec<i16> = screen.buffer.iter().copied().map(i16::from).collect();
    let sample_int_buf: Vec<i16> = sample.buffer
        .chunks_exact(sample_row_len)
        .step_by(h_step)
        .flat_map(|row|
                (0..sample_row_len)
                    .step_by(w_step)
                    .flat_map(|idx|
                        row[idx..idx + channels]
                            .iter()
                            .map(|&a| i16::from(a))
                    )
                    .collect::<Vec<i16>>()
            )
        .collect();

    let sample_mean: u8 = (sample_buf.iter().copied().map(u32::from).sum::<u32>() / sample_buf.len() as u32).try_into().unwrap();
    let mut screen_columns_sum: Vec<u32>;

    let mut min_diff = u32::MAX;
    let mut best_idx: usize = 0;

    let mut pos_y: usize = 0;
    while pos_y <= y_positions {
        screen_columns_sum = vec![0; screen_row_len];

        let max_y = pos_y + sample_h * screen_row_len;
        let mut y = pos_y;
        while y < max_y {
            for (cell, x) in screen_columns_sum.iter_mut().zip(y..) {
                *cell += screen_buf[x] as u32;
            }
            y += screen_row_len
        }

        let end_pos_x = pos_y + x_positions;
        let mut pos_x = pos_y;
        let mut x_num = 0;
        while pos_x <= end_pos_x {

            let screen_mean: u8 = (
                screen_columns_sum[x_num..x_num + sample_row_len].iter()
                .copied()
                .sum::<u32>() / sample_buf.len() as u32
            ).try_into().unwrap();
            let delta: i16 = i16::from(screen_mean) - i16::from(sample_mean);
            let diff_sum = match_window(
                &screen_int_buf,
                &sample_int_buf,
                pos_x,
                sample_row_len,
                w_step,
                h_step * screen_row_len,
                area,
                delta,
                channels,
                min_diff
            );
            if diff_sum < min_diff {
                min_diff = diff_sum;
                best_idx = pos_x;
            }
            pos_x += channels;
            x_num += channels;
        }
        pos_y += screen_row_len;
    }
    // println!("{}", min_diff as f32 / (checked_bytes * u8::MAX as usize) as f32);  // tolerance needed for the best match
    
    if TOLERANCE > min_diff as f32 / (sample_int_buf.len() * u8::MAX as usize) as f32 {
        let x = (best_idx % screen_row_len / channels).try_into().unwrap();
        let y = (best_idx / screen_row_len).try_into().unwrap();
        Some([x, y])
    } else {
        None
    }
}

#[inline(always)]
fn match_window(
    screen: &[i16],
    sample: &[i16],
    screen_row: usize,
    sample_row_len: usize,
    w_step: usize,
    h_step: usize,
    area: usize,
    delta: i16,
    channels: usize,
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
            for c in idx..idx + channels {
                diff_sum += (screen[c] - sample[channel] - delta).abs() as u32;
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
