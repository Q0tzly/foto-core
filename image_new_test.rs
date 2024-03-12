use foto_core;

#[test]
fn test_image_new() {
    let path = "tests/imgs/IMG_4141.CR2".to_string();
    println!("{:?}", foto_core::Image::new(path));
    assert_eq!(1, 1);
}
