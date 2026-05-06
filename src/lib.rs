const TOLERANCE: f32 = 0.05;



pub fn images_match(
    screen: &ImageView,
    sample: &ImageView,
    start_x: usize,
    start_y: usize,
) -> bool {
    assert_eq!(screen.channels, sample.channels);
    let channels = screen.channels;

    let screen_w = screen.width;
    let sample_w = sample.width;
    let sample_h = sample.height;

    let raw_screen = screen.buffer;
    let raw_sample = sample.buffer;
    
    let mut diff_sum: u32 = 0;
    for y in 0..sample_h {
        let screen_start = (start_y + y) * screen_w + start_x;
        let sample_start = y * sample_w;
        for x in 0..sample_w {
            for c in 0..channels {
                diff_sum += raw_screen[(screen_start + x) * channels + c].abs_diff(raw_sample[(sample_start + x) * sample.channels + c]) as u32;
            }
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

    let screen_w = screen.width as usize;
    let sample_w = sample.width as usize;

    let screen_h = screen.height as usize;
    let sample_h = sample.height as usize;

    let raw_screen = screen.buffer;
    let raw_sample = sample.buffer;

    let w_step = sample_w.isqrt(); // too big step
    let h_step = sample_h.isqrt();

    let mut min_diff = u32::MAX;
    let (mut best_x, mut best_y) = (0, 0);
    for y in 0..=screen_h - sample_h {
        for x in 0..=screen_w - sample_w {

            let mut diff_sum: u32 = 0;
            for sy in (0..sample_h).step_by(h_step) {
                let screen_start = (y + sy) * screen_w + x;
                let sample_start = sy * sample_w;
                for sx in (0..sample_w).step_by(w_step) {
                    let screen_px = (screen_start + sx) * channels;
                    let sample_px = (sample_start + sx) * channels;

                    for c in 0..channels {
                        diff_sum += raw_screen[screen_px + c].abs_diff(raw_sample[sample_px + c]) as u32;
                    }
                }
                if diff_sum > min_diff {
                    break;
                }
            }
            if diff_sum < min_diff {
                min_diff = diff_sum;
                (best_x, best_y) = (x, y);
            }
        }
    }
    let checked_bytes = (sample_w / w_step) * (sample_h / h_step) * channels;
    // println!("{}", min_diff as f32 / (checked_bytes * u8::MAX as usize) as f32);  // tolerance need
    
    return if TOLERANCE > min_diff as f32 / (checked_bytes * u8::MAX as usize) as f32 {
         Some((best_x, best_y))
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
        66,  66,  66,  66,  66,  66,  66,  66,  66,
        66,  66,  66,   0,   0,   0, 255, 255, 255,
        66,  66,  66, 255, 255, 255,   0,   0,   0,
        66,  66,  66, 100, 100, 100,  50,  50,  50,
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