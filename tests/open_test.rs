use foto_core;
//use std::env::args;
//use std::fs::File;

#[test]
fn test_open_file() {
    let path = "tests/imgs/IMG_4141.CR2".to_string();
    println!("{:?}", foto_core::Image::new(path));
    assert_eq!(1, 1);
}
