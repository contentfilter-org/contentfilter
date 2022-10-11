//  References
//   [1] https://www.hackerfactor.com/blog/index.php?/archives/529-Kind-of-Like-That.html
//   [2] https://github.com/benhoyt/dhash
//   [3] https://github.com/cetra3/dhash/blob/master/src/main.rs
//
use image::imageops::{grayscale, resize, FilterType};
use image::GenericImageView;


const IMAGE_SIZE: u32 = 8;
const PIXEL_COUNT: usize = (IMAGE_SIZE * IMAGE_SIZE) as usize;

pub fn dhash<I: GenericImageView + 'static>(img: &I) -> u64 {
    let gray_img = grayscale(img);
    let resized_img = resize(&gray_img, IMAGE_SIZE + 1, IMAGE_SIZE, FilterType::Triangle);
    let mut bits: [bool; PIXEL_COUNT] =[false; PIXEL_COUNT];
    let mut cur_index = 0;
    for i in 0..IMAGE_SIZE {
        for j in 0..IMAGE_SIZE {
            let left_pixel = resized_img.get_pixel(i, j);
            let right_pixel = resized_img.get_pixel(i + 1, j);
            bits[cur_index] = left_pixel[0] > right_pixel[0];
            cur_index += 1;
        }
    }

    let mut hash_value: u64 = 0;
    for i in 0..bits.len() {
        if bits[i] {
            hash_value += 1 << i;
        }
    }
    hash_value
}

pub fn hamming_distance(a: u64, b: u64) -> u32 {
    (a ^ b).count_ones()
}
