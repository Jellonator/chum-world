//! Used for BITMAP file conversion.
//! See https://github.com/Jellonator/chum-world/wiki/BITMAP for more information.

use crate::export::ChumExport;
use crate::format::TotemFormat;
use std::error::Error;
use std::io::{self, Read, Write};
use std::slice;
use image;
use image::Pixel;


// Image formats
const FORMAT_C4: u8 = 1;
const FORMAT_C8: u8 = 2;
const FORMAT_RGB565: u8 = 8;
const FORMAT_A3RGB565: u8 = 10;
const FORMAT_ARGB8888: u8 = 12;
const FORMAT_RGB888: u8 = 13;

// Palette format
const PALETTE_A3RGB5: u8 = 1;
const PALETTE_RGB565: u8 = 2;
const PALETTE_RGBA8888: u8 = 3;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

// TODO: Avoid unecessary panic!s

#[allow(non_snake_case)]
impl Color {
    /// Create a Color from an RGB565 value.
    pub fn from_RGB565(value: u16) -> Color {
        let red = ((value & 0b11111_000000_00000) >> 8) as u8;
        let green = ((value & 0b00000_111111_00000) >> 3) as u8;
        let blue = ((value & 0b00000_000000_11111) << 3) as u8;
        Color {
            r: red | (red >> 5),
            g: green | (green >> 6),
            b: blue | (blue >> 5),
            a: 255,
        }
    }

    /// Create a Color from an A3RGB5 value.
    /// This format in particular is special; the first bit determines
    /// the format of the rest of the Color.
    /// If the first bit is 0, then the format is RGB555 (15 bits),
    /// otherwise if the first bit is 1, then the format is A3RGB444 (15 bits).
    pub fn from_A3RGB5(value: u16) -> Color {
        if value & 0b10000000_00000000 != 0 {
            let red = ((value & 0b011111_00000_00000) >> 7) as u8;
            let green = ((value & 0b000000_11111_00000) >> 2) as u8;
            let blue = ((value & 0b000000_00000_11111) << 3) as u8;
            Color {
                r: red | (red >> 5),
                g: green | (green >> 5),
                b: blue | (blue >> 5),
                a: 255,
            }
        } else {
            let alpha = ((value & 0b0_111_0000_0000_0000) >> 7) as u8;
            let red = ((value & 0b0_000_1111_0000_0000) >> 4) as u8;
            let green = (value & 0b0_000_0000_1111_0000) as u8;
            let blue = ((value & 0b0_000_0000_0000_1111) << 4) as u8;
            Color {
                r: red | (red >> 4),
                g: green | (green >> 4),
                b: blue | (blue >> 4),
                a: alpha | (alpha >> 3) | (alpha >> 6),
            }
        }
    }

    /// Create a Color from an ARGB8888 value.
    pub fn from_ARGB8888(value: u32) -> Color {
        let alpha = ((value & 0b11111111_00000000_00000000_00000000) >> 24) as u8;
        let red = ((value & 0b00000000_11111111_00000000_00000000) >> 16) as u8;
        let green = ((value & 0b00000000_00000000_11111111_00000000) >> 8) as u8;
        let blue = (value & 0b00000000_00000000_00000000_11111111) as u8;
        return Color {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        };
    }

    /// Create a Color from an RGBA8888 value.
    pub fn from_RGBA8888(value: u32) -> Color {
        let red = ((value & 0b11111111_00000000_00000000_00000000) >> 24) as u8;
        let green = ((value & 0b00000000_11111111_00000000_00000000) >> 16) as u8;
        let blue = ((value & 0b00000000_00000000_11111111_00000000) >> 8) as u8;
        let alpha = (value & 0b00000000_00000000_00000000_11111111) as u8;
        return Color {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        };
    }

    /// Create a Color from an RGB888 value.
    pub fn from_RGB888(value: u32) -> Color {
        let red = ((value & 0b00000000_11111111_00000000_00000000) >> 16) as u8;
        let green = ((value & 0b00000000_00000000_11111111_00000000) >> 8) as u8;
        let blue = (value & 0b00000000_00000000_00000000_11111111) as u8;
        return Color {
            r: red,
            g: green,
            b: blue,
            a: 255,
        };
    }
}

/// The alpha level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlphaLevel {
    /// alpha is always 255
    Opaque,
    /// alpha is always either 0 or 255
    Bit,
    /// alpha can be any value
    Blend,
}

/// The BITMAP data. Contains colors for all pixels in the bitmap.
#[derive(Clone, Debug)]
pub struct Bitmap {
    data: Vec<Color>,
    alpha: AlphaLevel,
    width: u32,
    height: u32,
}

/// Convert a chunk index to an index into Bitmap's data array.
fn get_chunk_index(
    index: usize,
    blockwidth: usize,
    blockheight: usize,
    imagewidth: usize,
    _imageheight: usize,
) -> usize {
    let blocksize = blockwidth * blockheight;
    let blocks_per_row = imagewidth / blockwidth;
    let block_i = index % blocksize;
    let blockid = index / blocksize;
    let blockcol = blockid % blocks_per_row;
    let blockrow = blockid / blocks_per_row;
    let ix = blockcol * blockwidth + (block_i % blockwidth);
    let iy = blockrow * blockheight + (block_i / blockwidth);
    return iy * imagewidth + ix;
}

/// Arrange the pixel data into a Vector following bitmap chunk rules.
fn arrange_blocks<T>(
    data: Vec<T>,
    blockwidth: usize,
    blockheight: usize,
    imagewidth: usize,
    imageheight: usize,
) -> Vec<T>
where
    T: Default + Clone,
{
    if data.len() != imagewidth * imageheight {
        panic!();
    }
    if blockheight == 0 || blockwidth == 0 {
        panic!();
    }
    if imagewidth % blockwidth != 0 || imageheight % blockheight != 0 {
        panic!();
    }
    if imagewidth == blockwidth {
        return data;
    }
    let mut newdata = vec![T::default(); imagewidth * imageheight];
    for (i, col) in data.iter().enumerate() {
        newdata[get_chunk_index(i, blockwidth, blockheight, imagewidth, imageheight)] = col.clone();
    }
    newdata
}

/// Read a palette from the file.
/// The palette format and the number of colors must be provided.
fn read_palette<R: Read>(
    file: &mut R,
    fmt: TotemFormat,
    palette_format: u8,
    num: usize,
) -> io::Result<Vec<Color>> {
    match palette_format {
        PALETTE_A3RGB5 => {
            let mut palettedata = vec![0; num];
            fmt.read_u16_into(file, &mut palettedata)?;
            Ok(palettedata
                .into_iter()
                .map(|col| Color::from_A3RGB5(col))
                .collect())
        }
        PALETTE_RGB565 => {
            let mut palettedata = vec![0; num];
            fmt.read_u16_into(file, &mut palettedata)?;
            Ok(palettedata
                .into_iter()
                .map(|col| Color::from_RGB565(col))
                .collect())
        }
        PALETTE_RGBA8888 => {
            let mut palettedata = vec![0; num];
            fmt.read_u32_into(file, &mut palettedata)?;
            Ok(palettedata
                .into_iter()
                .map(|col| Color::from_RGBA8888(col))
                .collect())
        }
        _ => panic!(),
    }
}

/// Read the interleaved color format (FORMAT_ARGB8888)
fn read_u32_interleaved<R: Read>(
    file: &mut R,
    fmt: TotemFormat,
    num: usize,
) -> io::Result<Vec<Color>> {
    let mut res = Vec::with_capacity(num);
    if num % 16 != 0 {
        panic!();
    }

    for _ in 0..num / 16 {
        let mut buf = [0; 64];
        fmt.read_u8_into(file, &mut buf)?;
        for i in 0..16 {
            res.push(Color {
                r: buf[1 + i * 2],
                g: buf[32 + i * 2],
                b: buf[33 + i * 2],
                a: buf[0 + i * 2],
            });
        }
    }

    Ok(res)
}

impl Bitmap {
    /// Get the size
    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Get the width
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Get the height
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Get this bitmap's color data as a slice
    pub fn get_data(&self) -> &[Color] {
        &self.data
    }

    /// Get this bitmap's alpha level
    pub fn get_alpha_level(&self) -> AlphaLevel {
        self.alpha
    }

    /// Convert a 2d position to an index
    pub fn pos_to_index(&self, x: u32, y: u32) -> usize {
        return x as usize * self.width as usize + y as usize;
    }

    /// Read a Bitmap from a file
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<Bitmap> {
        let width: u32 = fmt.read_u32(file)?;
        let height: u32 = fmt.read_u32(file)?;
        fmt.skip_n_bytes(file, 4)?;
        let format: u8 = fmt.read_u8(file)?;
        let _flags: u8 = fmt.read_u8(file)?;
        let palette_format: u8 = fmt.read_u8(file)?;
        let opacity_level: u8 = fmt.read_u8(file)?;
        let _unk: u8 = fmt.read_u8(file)?;
        let _filter: u8 = fmt.read_u8(file)?;
        println!(
            "Format: ({} {} {} {} {}",
            format, _flags, palette_format, opacity_level, _filter
        );
        // TODO: Handle irregular image sizes for arrange_blocks
        let data: Vec<Color> = match format {
            FORMAT_C4 => {
                let mut indices = vec![0; (width * height) as usize];
                fmt.read_u4_into(file, &mut indices)?;
                let palette = read_palette(file, fmt, palette_format, 16)?;
                let data: Vec<Color> = indices.into_iter().map(|i| palette[i as usize]).collect();
                arrange_blocks(data, 8, 8, width as usize, height as usize)
            }
            FORMAT_C8 => {
                let mut indices = vec![0; (width * height) as usize];
                fmt.read_u8_into(file, &mut indices)?;
                let palette = read_palette(file, fmt, palette_format, 256)?;
                let data: Vec<Color> = indices.into_iter().map(|i| palette[i as usize]).collect();
                arrange_blocks(data, 8, 4, width as usize, height as usize)
            }
            FORMAT_RGB565 => {
                let mut data = vec![0; (width * height) as usize];
                fmt.read_u16_into(file, &mut data)?;
                let colordata = data
                    .into_iter()
                    .map(|col| Color::from_RGB565(col))
                    .collect();
                arrange_blocks(colordata, 4, 4, width as usize, height as usize)
            }
            FORMAT_A3RGB565 => {
                let mut data = vec![0; (width * height) as usize];
                fmt.read_u16_into(file, &mut data)?;
                let colordata = data
                    .into_iter()
                    .map(|col| Color::from_A3RGB5(col))
                    .collect();
                arrange_blocks(colordata, 4, 4, width as usize, height as usize)
            }
            FORMAT_ARGB8888 => {
                let data = read_u32_interleaved(file, fmt, (width * height) as usize)?;
                arrange_blocks(data, 4, 4, width as usize, height as usize)
            }
            FORMAT_RGB888 => {
                let mut data = vec![0; (width * height) as usize];
                fmt.read_u24_into(file, &mut data)?;
                data.into_iter()
                    .map(|col| Color::from_RGB888(col))
                    .collect()
                // linear format, no blocks necessary
                // TODO: Handle weird format
            }
            _ => panic!(),
        };
        Ok(Bitmap {
            data,
            alpha: match opacity_level {
                0 => AlphaLevel::Opaque,
                1 => AlphaLevel::Bit,
                2 => AlphaLevel::Blend,
                _ => panic!(),
            },
            width,
            height,
        })
    }

    /// Read a TMesh from data
    pub fn read_data(data: &[u8], fmt: TotemFormat) -> io::Result<Bitmap> {
        Bitmap::read_from(&mut data.as_ref(), fmt)
    }
}

impl image::GenericImageView for Bitmap {
    type Pixel = image::Rgba<u8>;
    type InnerImageView = Self;
    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    fn bounds(&self) -> (u32, u32, u32, u32) {
        (0, 0, self.width, self.height)
    }
    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        let col = &self.data[self.pos_to_index(x, y)];
        let ptr = &col.r as *const u8;
        unsafe {
            // this is fine
            *Self::Pixel::from_slice(slice::from_raw_parts(ptr, 4))
        }
    }
    fn inner(&self) -> &Self::InnerImageView {
        self
    }
}
impl image::GenericImage for Bitmap {
    type InnerImage = Self;
    fn get_pixel_mut(&mut self, x: u32, y: u32) -> &mut Self::Pixel {
        let idx = self.pos_to_index(x, y);
        let col = &mut self.data[idx];
        let ptr = &mut col.r;
        unsafe {
            // this is fine
            Self::Pixel::from_slice_mut(slice::from_raw_parts_mut(ptr, 4))
        }
    }
    fn put_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) {
        let idx = self.pos_to_index(x, y);
        self.data[idx] = Color {
            r: pixel[0],
            g: pixel[1],
            b: pixel[2],
            a: pixel[3],
        }
    }
    fn blend_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) {
        self.get_pixel_mut(x, y).blend(&pixel);
    }
    fn inner_mut(&mut self) -> &mut Self::InnerImage {
        return self;
    }
}

impl ChumExport for Bitmap {
    fn export<W>(&self, writer: &mut W) -> Result<(), Box<dyn Error>>
    where
        W: Write,
    {
        let encoder = image::png::PNGEncoder::new(writer);
        unsafe {
            let ptr = self.data.as_ptr() as *const u8;
            encoder.encode(
                slice::from_raw_parts(ptr, self.data.len() * 4),
                self.width,
                self.height,
                image::ColorType::Rgba8,
            )?;
        }
        Ok(())
    }
}
