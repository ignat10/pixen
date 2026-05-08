const TOLERANCE: f32 = 0.05;



pub fn images_match(
    screen: &ImageView,
    sample: &ImageView,
    start_x: usize,
    start_y: usize,
) -> bool {
    assert_eq!(screen.channels, sample.channels);
    let channels = screen.channels;

    let screen_w = screen.width * channels;
    let sample_w = sample.width * channels;
    let sample_h = sample.height;

    let raw_screen = screen.buffer;
    let raw_sample = sample.buffer;
    
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
    screen: &ImageView,
    sample: &ImageView,
) -> Option<(usize, usize)> {
    assert_eq!(screen.channels, sample.channels);
    let channels = screen.channels;

    let screen_w = screen.width;
    let sample_w = sample.width;

    let sample_h = sample.height;

    let screen_row_len = screen_w * channels;
    let sample_row_len = sample_w * channels;

    let raw_screen = screen.buffer;
    let raw_sample = sample.buffer;

    let w_step = sample_w.isqrt() * channels;
    let h_step = sample_h.isqrt();

    let mut min_diff = u32::MAX;
    let mut best_idx: usize = 0;
    for start_row_idx in (0..=raw_screen.len() - sample_h * screen_row_len).step_by(screen_row_len)  {
        for start_idx in (start_row_idx..=start_row_idx + screen_row_len - sample_row_len).step_by(channels) {
            let mut diff_sum: u32 = 0;
            for (y_idx, sample_start) in 
            (start_idx..).step_by(screen_row_len)
            .zip((0..raw_sample.len()).step_by(sample_row_len))
            .step_by(h_step)
            {
                for (x_idx, sample_idx) in 
                (y_idx..)
                .zip(sample_start..sample_start + sample_row_len)
                .step_by(w_step)
                {
                    for c in 0..channels {
                        diff_sum += raw_screen[x_idx + c].abs_diff(raw_sample[sample_idx + c]) as u32;
                    }
                }
                if diff_sum > min_diff {
                    break;
                }
            }
            if diff_sum < min_diff {
                min_diff = diff_sum;
                best_idx = start_idx;
            }
        }
    }
    let checked_bytes = (sample_row_len / w_step) * (sample_h / h_step) * channels;
    // println!("{}", min_diff as f32 / (checked_bytes * u8::MAX as usize) as f32);  // tolerance needed for the best match
    
    return if TOLERANCE > min_diff as f32 / (checked_bytes * u8::MAX as usize) as f32 {
        let x = best_idx % screen_row_len / channels;
        let y = best_idx / screen_row_len;
         Some((x, y))
    } else {
        None
    }
}


pub struct ImageView<'a> {
    pub buffer: &'a [u8],
    pub channels: usize,
    pub width: usize,
    pub height: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    const SCREEN_BUFFER: &[u8] = &[
        66,  66,  66,   66,  66,  66,   66,  66,  66,
        66,  66,  66,    0,   0,   0,  255, 255, 255,
        66,  66,  66,  255, 255, 255,    0,   0,   0,
        66,  66,  66,  100, 100, 100,   50,  50,  50,
    ];

    const SAMPLE_BUFFER: &[u8] = &[
          5,   5,   5, 250, 250, 250,
        250, 250, 250,   5,   5,   5,
        105,  95, 100,  45,  55,  50,
    ];

    const DIFFERENT_SAMPLE_BUFFER: &[u8] = &[
        255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255,
    ];

    #[test]
    fn test_images_match() {
        let screen = ImageView {
            buffer: SCREEN_BUFFER,
            channels: 3,
            width: 3,
            height: 4,
        };
        let sample = ImageView {
            buffer: SAMPLE_BUFFER,
            channels: 3,
            width: 2,
            height: 3,
        };

        assert!(!images_match(&screen, &sample, 0, 0));
        assert!(!images_match(&screen, &sample, 0, 1));
        assert!(!images_match(&screen, &sample, 1, 0));
        assert!(images_match(&screen, &sample, 1, 1));
    }

    #[test]
    fn test_find_sample_found() {
        let screen = ImageView {
            buffer: SCREEN_BUFFER,
            channels: 3,
            width: 3,
            height: 4,
        };
        let sample = ImageView {
            buffer: SAMPLE_BUFFER,
            channels: 3,
            width: 2,
            height: 3,
        };
        assert_eq!(find_sample(&screen, &sample), Some((1, 1)));
    }

    #[test]
    fn test_find_sample_not_found() {
        let screen = ImageView {
            buffer: SCREEN_BUFFER,
            channels: 3,
            width: 3,
            height: 4,
        };
        let sample = ImageView {
            buffer: DIFFERENT_SAMPLE_BUFFER,
            channels: 3,
            width: 2,
            height: 3,
        };
        assert_eq!(find_sample(&screen, &sample), None);
    }
}