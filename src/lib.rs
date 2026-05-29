const TOLERANCE: f32 = 0.1;



pub fn images_match(
    screen: &Image,
    sample: &Image,
    coords: [u16; 2]
) -> bool {
    assert_eq!(screen.channels, sample.channels, "screen channels = {}, sample channels = {}", screen.channels, sample.channels);
    assert!(screen.width >= sample.width, "screen width = {}, sample width = {}", screen.width, sample.width);
    assert!(screen.height >= sample.height, "screen height = {}, sample height = {}", screen.height, sample.height);

    let [start_x, start_y] = coords;
    let start_x: usize = start_x.into();
    let start_y: usize = start_y.into();

    assert!(start_x <= screen.width - sample.width);
    assert!(start_y <= screen.height - sample.height);

    let channels = screen.channels;

    let screen_w = screen.width * channels;
    let sample_w = sample.width * channels;
    let sample_h = sample.height;

    let raw_screen = &screen.buffer;
    let raw_sample = &sample.buffer;
    
    let mut diff_sum: u32 = 0;
    let mut sample_idx = 0;
    let corner_idx = start_y * screen_w + start_x * channels;
    for start_row_idx in (corner_idx..corner_idx + sample_h * screen_w).step_by(screen_w) {
        for screen_idx in start_row_idx..start_row_idx + sample_w {
            diff_sum += raw_screen[screen_idx].abs_diff(raw_sample[sample_idx]) as u32;
            sample_idx += 1;
        }
    };
    TOLERANCE > diff_sum as f32 / (raw_sample.len() * u8::MAX as usize) as f32
}


pub fn find_sample(
    screen: &Image,
    sample: &Image,
) -> Option<[u16; 2]> {
    assert_eq!(screen.channels, sample.channels);
    let screen_buf = &screen.buffer;
    let sample_buf = &sample.buffer;

    let screen_int_buf: Vec<i16> = screen.buffer.iter().copied().map(i16::from).collect();
    let sample_int_buf: Vec<i16> = sample.buffer.iter().copied().map(i16::from).collect();

    let screen_w = screen.width;
    let sample_w = sample.width;

    let screen_h = screen.height;
    let sample_h = sample.height;

    let channels = screen.channels;

    let screen_row_len = screen_w * channels;
    let sample_row_len = sample_w * channels;

    let x_positions = screen_row_len - sample_row_len;
    let y_positions = (screen_h - sample_h) * screen_row_len;

    let w_step = sample_w.isqrt() * channels;
    let h_step = sample_h.isqrt();

    let sample_mean: u8 = (sample_buf.iter().copied().map(u32::from).sum::<u32>() / sample_buf.len() as u32).try_into().unwrap();
    let mut screen_columns_sum: Vec<u16>;

    let mut min_diff = u32::MAX;
    let mut best_idx: usize = 0;

    let mut pos_y: usize = 0;
    while pos_y <= y_positions {
        screen_columns_sum = vec![0; screen_row_len];

        let max_y = pos_y + sample_h * screen_row_len;
        let mut y = pos_y;
        while y < max_y {
            for (cell, x) in screen_columns_sum.iter_mut().zip(y..) {
                *cell += screen_buf[x] as u16;
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
                .map(u32::from)
                .sum::<u32>() / sample_buf.len() as u32
            ).try_into().unwrap();
            let delta: i16 = i16::from(screen_mean) - i16::from(sample_mean);

            let mut diff_sum: u32 = 0;
            let mut screen_row = pos_x;
            let mut sample_row = 0;
            while sample_row < sample_buf.len() {
                unsafe {
                    let screen_x = screen_int_buf.as_ptr().add(screen_row);
                    let sample_x = sample_int_buf.as_ptr().add(sample_row);
                    let mut i = 0;
                    while i < sample_row_len {
                        for c in i..i + channels {
                            diff_sum += (*screen_x.add(c) - *sample_x.add(c) - delta).abs() as u32;
                        }
                        i += w_step;
                    }
                    if diff_sum > min_diff {
                        break;
                    }
                    screen_row += screen_row_len * h_step;
                    sample_row += sample_row_len * h_step;
                }
            }
            if diff_sum < min_diff {
                min_diff = diff_sum;
                best_idx = pos_x;
            }
            pos_x += channels;
            x_num += channels;
        }
        pos_y += screen_row_len;
    }
    let checked_bytes = (sample_row_len / w_step) * (sample_h / h_step) * channels;
    // println!("{}", min_diff as f32 / (checked_bytes * u8::MAX as usize) as f32);  // tolerance needed for the best match
    
    if TOLERANCE > min_diff as f32 / (checked_bytes * u8::MAX as usize) as f32 {
        let x = (best_idx % screen_row_len / channels).try_into().unwrap();
        let y = (best_idx / screen_row_len).try_into().unwrap();
        Some([x, y])
    } else {
        None
    }
}


pub struct Image {
    pub(crate) buffer: Vec<u8>,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) channels: usize,
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
            }
        )
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    pub fn channels(&self) -> usize { self.channels }

    pub fn as_raw(&self) -> &Vec<u8> { &self.buffer }
}
