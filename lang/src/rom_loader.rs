use image::ImageReader;
use std::error::Error;
use std::path::Path;
use std::{fs, io};

struct Rom(Vec<u8>);
impl Rom {
    pub fn parse_p8_png(self) -> Cartridge {
        let (left, right) = self.0.split_at_checked(0x4300).unwrap();
        let data = left.to_vec();
        let code = right.to_vec();

        Cartridge { data, code }
    }

    pub fn parse_p8(self) -> Cartridge {
        // This method needs to be implemented
        Cartridge::default()
    }
}

pub struct Cartridge {
    code: Vec<u8>,
    data: Vec<u8>,
}

impl Default for Cartridge {
    fn default() -> Self {
        Cartridge {
            code: Vec::new(),
            data: Vec::new(),
        }
    }
}

impl Cartridge {
    pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let path = Path::new(url);
        if !path.exists() {
            Err("No such file found in".into())
        } else if path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .ends_with(".p8.png")
        {
            Ok(read_p8_png(path)?.parse_p8_png().decompress_new_format()?)
        } else if path.file_name().unwrap().to_string_lossy().ends_with(".p8") {
            Ok(read_p8(path)?.parse_p8()) // Not supported yet
        } else {
            Err("File not found".to_string().into())
        }
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn code(&self) -> &Vec<u8> {
        &self.code
    }

    pub fn decompress_new_format(mut self) -> io::Result<Self> {
        let code: &[u8] = &self.code; // from here start the lua code
        if &code[0..4] != b"\x00pxa" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Invalid magic header {:?}, maybe its the old compression format?",
                    &code[0..4]
                ),
            ));
        } // Check the magic header, see if it is the new format

        let decompressed_len = u16::from_be_bytes([code[4], code[5]]) as usize; // MSB
        let total_len = u16::from_be_bytes([code[6], code[7]]) as usize;

        let compressed_data = &code[8..total_len];
        let mut output = Vec::with_capacity(decompressed_len);

        let mut mtf: Vec<u8> = (0u8..=255).collect();
        let mut bit_cursor = 0;

        let mut _read_bit = |bit_cursor: &mut usize| -> u8 {
            let byte_index = *bit_cursor / 8;
            let bit_index = *bit_cursor % 8;
            let bit = (compressed_data[byte_index] >> bit_index) & 1;
            *bit_cursor += 1;
            bit
        };

        let mut _read_bits = |bit_cursor: &mut usize, n: usize| -> u32 {
            let mut result = 0;
            for i in 0..n {
                result |= (_read_bit(bit_cursor) as u32) << i;
            }
            result
        };

        while output.len() < decompressed_len {
            let header = _read_bit(&mut bit_cursor);

            if header == 1 {
                // Move-to-front
                let mut unary = 0;
                while _read_bit(&mut bit_cursor) == 1 {
                    unary += 1;
                }
                let unary_mask = (1 << unary) - 1;
                let idx = _read_bits(&mut bit_cursor, 4 + unary) + ((unary_mask as u32) << 4);

                let idx = idx as usize;
                if idx >= mtf.len() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "MTF index out of bounds",
                    ));
                }

                let byte = mtf[idx];
                output.push(byte);

                mtf.remove(idx);
                mtf.insert(0, byte);
            } else {
                // LZ-style copy or uncompressed
                let offset_bits = match _read_bit(&mut bit_cursor) {
                    1 => {
                        if _read_bit(&mut bit_cursor) == 1 {
                            5
                        } else {
                            10
                        }
                    }
                    _ => 15,
                };

                let offset = _read_bits(&mut bit_cursor, offset_bits) as usize + 1;

                // Uncompressed block
                if offset_bits == 10 && offset == 1 {
                    loop {
                        let byte = _read_bits(&mut bit_cursor, 8) as u8;
                        if byte == 0 {
                            break;
                        }
                        output.push(byte);
                    }
                    continue;
                }

                // Length decoding
                let mut length = 3;
                loop {
                    let part = _read_bits(&mut bit_cursor, 3) as usize;
                    length += part;
                    if part != 7 {
                        break;
                    }
                }

                // Copy from earlier output
                for _i in 0..length {
                    let copy_from = output.len().checked_sub(offset).ok_or_else(|| {
                        io::Error::new(io::ErrorKind::UnexpectedEof, "Invalid offset")
                    })?;
                    output.push(output[copy_from]);
                }
            }
        }
        self.code = output;

        Ok(self)
    }
}

fn read_p8_png(path: &Path) -> Result<Rom, Box<dyn Error>> {
    let img = ImageReader::open(path)?.decode()?.to_rgba8();

    let mut result = Vec::new();

    let pixels = img.pixels().take(0x7fff);

    for pixel in pixels {
        let [r, g, b, a] = pixel.0;

        let byte = ((a & 0b11) << 6) | ((r & 0b11) << 4) | ((g & 0b11) << 2) | (b & 0b11);

        result.push(byte);
    }

    Ok(Rom(result))
}

fn read_p8(path: &Path) -> Result<Rom, Box<dyn Error>> {
    if !path.exists() {
        return Err("No such file found".into());
    } else if !path.file_name().unwrap().to_string_lossy().ends_with(".p8") {
        return Err("File is not .p8".into());
    }

    let s = fs::read(path)?;
    Ok(Rom(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_p8_png() {
        let b = Cartridge::new("../examples/ppg-1.p8.png")
            .unwrap()
            .code
            .windows(b"pico".len())
            .any(|window| window == b"pico");

        assert!(b, "Failed to read .p8.png file, algo incorrect!");
    }

    #[test]
    fn test_decompress_p8_png() {
        let cart = Cartridge::new("../examples/ppg-1.p8.png").unwrap();

        let s = String::from_utf8_lossy(&cart.code);

        assert!(s.ends_with("end"));
    }
}
