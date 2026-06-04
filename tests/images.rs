use std::env::var;
use std::path::PathBuf;
use std::sync::LazyLock;

use stb_image::image;

use pixen::Image;


static TEST_DATA: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(var("CARGO_MANIFEST_DIR").unwrap()).join("tests/data")
});


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


pub static SCREEN: LazyLock<Image> = LazyLock::new(|| {
    Image::new(
        SCREEN_BUFFER.into(),
        3,
        4,
        3
    ).unwrap()
});

pub static SAMPLE: LazyLock<Image> = LazyLock::new(|| {
    Image::new(
        SAMPLE_BUFFER.into(),
        2,
        3,
        3
    ).unwrap()
});


pub static DIFFERENT_SAMPLE: LazyLock<Image> = LazyLock::new(|| {
    Image::new(
        DIFFERENT_SAMPLE_BUFFER.into(),
        2,
        3,
        3
    ).unwrap()
});

fn load_from_data(p: impl AsRef<std::path::Path>) -> Image {
    let path = TEST_DATA.join(p);
    let image = image::load(&path);
    match image {
        image::LoadResult::ImageU8(image) => {
            Image::new(
                image.data,
                image.width,
                image.height,
                image.depth
            ).unwrap()
        },
        image::LoadResult::ImageF32(_) => panic!("Invalid format {}", path.to_str().unwrap()),
        image::LoadResult::Error(e) => panic!("{}", e),
    }
}

pub static PNG_SCREEN: LazyLock<Image> = LazyLock::new(|| {
    load_from_data("screen.png")
});

pub static PNG_SAMPLE: LazyLock<Image> = LazyLock::new(|| {
    load_from_data("sample.png")
});

pub static PNG_NOT_SAMPLE: LazyLock<Image> = LazyLock::new(|| {
    load_from_data("not_present_sample.png")
});

pub static NTH_SCREEN: LazyLock<Image> = LazyLock::new(|| {
    load_from_data("nth_screen.png")
});
