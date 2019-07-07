use std::{
    fs::File,
    path::Path,
    io::Write
};
use core::fmt::Write as FmtWrite;

use itertools::Itertools;

pub fn create<P: AsRef<Path>>(output_path: P, image_x: usize, image_y: usize, bytes: &[u8]) {
    let mut buffer = String::new();
    for row in bytes.chunks(3).chunks(image_x).into_iter() {
        for pixel in row {
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            buffer.write_str(&format!("{} {} {}\n", r, g, b));
        }
    }

    let mut output = File::create(output_path).unwrap();
    output.write_all(&[]);

    {
        let header = format!("P3\n{} {}\n255\n", image_x, image_y);
        output.write(header.as_bytes());
    }

    output.write(buffer.as_bytes());
}