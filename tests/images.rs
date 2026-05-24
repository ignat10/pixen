use std::sync::LazyLock;
use pixen::Image;

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


pub(crate) static SCREEN: LazyLock<Image> = LazyLock::new(|| {
    Image::new(
        SCREEN_BUFFER.into(),
        3u8,
        4u8,
        3u8
    )
});

pub(crate) static SAMPLE: LazyLock<Image> = LazyLock::new(|| {
    Image::new(
        SAMPLE_BUFFER.into(),
        2u8,
        3u8,
        3u8
    )
});


pub(crate) static DIFFERENT_SAMPLE: LazyLock<Image> = LazyLock::new(|| {
    Image::new(
        DIFFERENT_SAMPLE_BUFFER.into(),
        2u8,
        3u8,
        3u8
    )
});
