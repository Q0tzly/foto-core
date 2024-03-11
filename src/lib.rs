use bayer::*;
use rawloader::*;

use std::fs::File;
use std::{array, io, path::PathBuf, process};

pub struct Image {
    //data: Vec<u8>, //rgb data
    //format: String,
    //hash: String,
}

impl Image {
    pub fn new(path: String) {
        //-> Result<Self, io::Error> {
        let rgb_formats = ["jpg", "jpeg", "png", "bmp", "tiff", "tif", "gif", "webp"];
        let raw_formats = [
            "mrw", "arw", "srf", "sr2", "mef", "orf", "srw", "erf", "kdc", "dcs", "rw2", "raf",
            "dcr", "dng", "pef", "crw", "iiq", "3fr", "nrw", "nef", "mos", "cr2", "ari",
        ];

        /*let format: String = PathBuf::from(path.clone())
            .extension()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        */

        /*
        data <= path.{rawloader.bayer}
        */

        let mut file = File::open(path).unwrap(); //?;
        if let Ok(data) = rawloader::decode(&mut file) {
            println!("{:?}", data);
        };

        //let format = String::new();
        //let hash = String::new();

        //Ok(Image { format, hash })
    }
}
