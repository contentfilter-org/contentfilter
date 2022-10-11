include!("../src/algo/image/dhash.rs");


#[test]
fn hamming_distance_1() {
    assert_eq!(hamming_distance(1 as u64, 1 as u64), 0);
    assert_eq!(hamming_distance(0 as u64, 1 as u64), 1);
    assert_eq!(hamming_distance(1 as u64, 2 as u64), 2);
    assert_eq!(hamming_distance(0 as u64, 0 as u64), 0);
    assert_eq!(hamming_distance(100 as u64, 999 as u64), 5);
}

#[test]
fn dhash_1() {
    let img_1 = image::open("./tests/resources/dhash/grape1.jpeg").expect("cound not fin image");
    let img_2 = image::open("./tests/resources/dhash/grape2.jpeg").expect("cound not fin image");
    let hash_1 = dhash(&img_1);
    let hash_2 = dhash(&img_2);
    let dist = hamming_distance(hash_1, hash_2);
    assert_eq!(dist, 15);
}

#[test]
fn dhash_2() {
    let img_1 = image::open("./tests/resources/dhash/grape2.jpeg").expect("cound not fin image");
    let img_2 = image::open("./tests/resources/dhash/grape3.jpeg").expect("cound not fin image");
    let hash_1 = dhash(&img_1);
    let hash_2 = dhash(&img_2);
    let dist = hamming_distance(hash_1, hash_2);
    assert_eq!(dist, 7);
}
