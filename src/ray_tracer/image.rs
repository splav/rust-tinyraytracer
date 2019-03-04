use std::cmp;

use jpeg_decoder::Decoder;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

use super::Vec3f;

#[derive(Debug, Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Vec3f>,
}

impl Image {
    pub fn load(file_name: &str) -> Self {
        let file = File::open(file_name).expect("failed to open image");
        let mut decoder = Decoder::new(BufReader::new(file));
        let pixels = decoder.decode().expect("failed to decode image");
        let metadata = dbg!(decoder.info().expect("failed to obtain metadata"));

        let float_data: Vec<_> = pixels.iter().map(|v| (f32::from(*v)) / 255.).collect();
        let buffer = float_data[..]
            .chunks_exact(3)
            .map(|c| Vec3f::from_column_slice(c))
            .collect();
        Self {
            width: metadata.width as usize,
            height: metadata.height as usize,
            buffer,
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let mut file = BufWriter::new(File::create("out.ppm")?);

        write!(file, "P6\n{} {}\n255\n", self.width, self.height)?;
        for i in 0..self.height * self.width {
            let mut point = self.buffer[i];
            let max = point.max();
            if max > 1.0 {
                point /= max
            }
            for b in point.iter() {
                file.write_all(&[cmp::max(cmp::min((255. * b) as u8, 255), 0)])?;
            }
        }
        Ok(())
    }
}
