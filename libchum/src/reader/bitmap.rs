//! Used for BITMAP file conversion.
//! See https://github.com/Jellonator/chum-world/wiki/BITMAP for more information.

use crate::format::TotemFormat;
use crate::util;
pub use image;
use imagequant;
use std::error::Error;
use std::io::{self, BufRead, Read, Seek, Write};
use std::slice;

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

    pub fn to_RGB565(&self) -> u16 {
        let red = (self.r >> 3) as u16;
        let green = (self.g >> 2) as u16;
        let blue = (self.b >> 3) as u16;
        return (red << 11) | (green << 5) | blue;
    }

    /// Create a Color from an A3RGB5 value.
    /// This format in particular is special; the first bit determines
    /// the format of the rest of the Color.
    /// If the first bit is 1, then the format is RGB555 (15 bits),
    /// otherwise if the first bit is 0, then the format is A3RGB444 (15 bits).
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

    pub fn to_A3RGB5(&self) -> u16 {
        if self.a < 255 {
            let alpha = (self.a >> 5) as u16;
            let red = (self.r >> 4) as u16;
            let green = (self.g >> 4) as u16;
            let blue = (self.b >> 4) as u16;
            0x0000 | (alpha << 12) | (red << 8) | (green << 4) | blue
        } else {
            let red = (self.r >> 3) as u16;
            let green = (self.g >> 3) as u16;
            let blue = (self.b >> 3) as u16;
            0x8000 | (red << 10) | (green << 5) | blue
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

    pub fn to_RGBA8888(&self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
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

// The alpha level.
chum_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AlphaLevel {
        // alpha is always 255
        Opaque,
        // alpha is always either 0 or 255
        Bit,
        // alpha can be any value
        Blend,
    }
}

impl AlphaLevel {
    pub fn as_u8(&self) -> u8 {
        use AlphaLevel::*;
        match self {
            Opaque => 0,
            Bit => 1,
            Blend => 2,
        }
    }

    pub fn from_u8(value: u8) -> Option<AlphaLevel> {
        use AlphaLevel::*;
        match value {
            0 => Some(Opaque),
            1 => Some(Bit),
            2 => Some(Blend),
            _ => None,
        }
    }
}

/// Palette Format
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaletteFormat {
    RGB5A3,   // 1
    RGB565,   // 2
    RGBA8888, // 3
}

impl PaletteFormat {
    pub fn from_format(format: u8) -> Option<PaletteFormat> {
        match format {
            PALETTE_A3RGB5 => Some(PaletteFormat::RGB5A3),
            PALETTE_RGB565 => Some(PaletteFormat::RGB565),
            PALETTE_RGBA8888 => Some(PaletteFormat::RGBA8888),
            _ => None,
        }
    }

    pub fn get_format(&self) -> u8 {
        use PaletteFormat::*;
        match self {
            RGB5A3 => 1,
            RGB565 => 2,
            RGBA8888 => 3,
        }
    }

    pub fn get_color(&self, value: u32) -> Color {
        match *self {
            PaletteFormat::RGB5A3 => Color::from_A3RGB5(value as u16),
            PaletteFormat::RGB565 => Color::from_RGB565(value as u16),
            PaletteFormat::RGBA8888 => Color::from_RGBA8888(value),
        }
    }
}

#[derive(Clone)]
pub struct PaletteC4 {
    format: PaletteFormat,
    data: [u32; 16],
}

#[derive(Clone)]
pub struct PaletteC8 {
    format: PaletteFormat,
    data: [u32; 256],
}

impl PaletteC4 {
    pub fn new_empty(format: u8) -> Option<PaletteC4> {
        Some(PaletteC4 {
            data: [0; 16],
            format: PaletteFormat::from_format(format)?,
        })
    }

    pub fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
        use PaletteFormat::*;
        match self.format {
            RGB5A3 | RGB565 => {
                for i in 0..16 {
                    fmt.write_u16(writer, self.data[i] as u16)?;
                }
                for _ in 0..16 {
                    fmt.write_u16(writer, 0xFFFF)?;
                }
            }
            RGBA8888 => {
                for i in 0..16 {
                    fmt.write_u32(writer, self.data[i])?;
                }
            }
        }
        Ok(())
    }

    pub fn get_format(&self) -> u8 {
        self.format.get_format()
    }

    pub fn get_color(&self, index: u8) -> Color {
        match self.data.get(index as usize) {
            Some(val) => self.format.get_color(*val),
            None => panic!(),
        }
    }

    pub fn read_palette<R: Read>(
        ptype: u8,
        file: &mut R,
        fmt: TotemFormat,
    ) -> io::Result<PaletteC4> {
        match ptype {
            1 => {
                let mut palettedata = [0u16; 16];
                fmt.read_u16_into(file, &mut palettedata)?;
                let mut retdata = [0u32; 16];
                for i in 0..palettedata.len() {
                    retdata[i] = palettedata[i] as u32;
                }
                Ok(PaletteC4 {
                    format: PaletteFormat::RGB5A3,
                    data: retdata,
                })
            }
            2 => {
                let mut palettedata = [0u16; 16];
                fmt.read_u16_into(file, &mut palettedata)?;
                let mut retdata = [0u32; 16];
                for i in 0..palettedata.len() {
                    retdata[i] = palettedata[i] as u32;
                }
                Ok(PaletteC4 {
                    format: PaletteFormat::RGB565,
                    data: retdata,
                })
            }
            3 => {
                let mut palettedata = [0u32; 16];
                fmt.read_u32_into(file, &mut palettedata)?;
                Ok(PaletteC4 {
                    format: PaletteFormat::RGBA8888,
                    data: palettedata,
                })
            }
            _ => panic!(),
        }
    }
}

impl PaletteC8 {
    pub fn new_empty(format: u8) -> Option<PaletteC8> {
        Some(PaletteC8 {
            data: [0; 256],
            format: PaletteFormat::from_format(format)?,
        })
    }

    pub fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
        use PaletteFormat::*;
        match self.format {
            RGB5A3 | RGB565 => {
                for i in 0..256 {
                    fmt.write_u16(writer, self.data[i] as u16)?;
                }
                for _ in 0..256 {
                    fmt.write_u16(writer, 0xFFFF)?;
                }
            }
            RGBA8888 => {
                for i in 0..256 {
                    fmt.write_u32(writer, self.data[i])?;
                }
            }
        }
        Ok(())
    }

    pub fn get_format(&self) -> u8 {
        self.format.get_format()
    }

    pub fn get_color(&self, index: u8) -> Color {
        match self.data.get(index as usize) {
            Some(val) => self.format.get_color(*val),
            None => panic!(),
        }
    }

    pub fn read_palette<R: Read>(
        ptype: u8,
        file: &mut R,
        fmt: TotemFormat,
    ) -> io::Result<PaletteC8> {
        match ptype {
            PALETTE_A3RGB5 => {
                let mut palettedata = [0u16; 256];
                fmt.read_u16_into(file, &mut palettedata)?;
                let mut retdata = [0u32; 256];
                for i in 0..palettedata.len() {
                    retdata[i] = palettedata[i] as u32;
                }
                Ok(PaletteC8 {
                    format: PaletteFormat::RGB5A3,
                    data: retdata,
                })
            }
            PALETTE_RGB565 => {
                let mut palettedata = [0u16; 256];
                fmt.read_u16_into(file, &mut palettedata)?;
                let mut retdata = [0u32; 256];
                for i in 0..palettedata.len() {
                    retdata[i] = palettedata[i] as u32;
                }
                Ok(PaletteC8 {
                    format: PaletteFormat::RGB565,
                    data: retdata,
                })
            }
            PALETTE_RGBA8888 => {
                let mut palettedata = [0u32; 256];
                fmt.read_u32_into(file, &mut palettedata)?;
                Ok(PaletteC8 {
                    format: PaletteFormat::RGBA8888,
                    data: palettedata,
                })
            }
            _ => panic!(),
        }
    }
}

/// Image Format
#[derive(Clone)]
pub enum BitmapFormat {
    C4(Vec<u8>, PaletteC4),    // 1
    C8(Vec<u8>, PaletteC8),    // 2
    RGB565(Vec<u16>),          // 8
    RGB5A3(Vec<u16>),          // 10
    RGBA8888(Vec<Color>),      // 12
    RGB888(Vec<(u8, u8, u8)>), // 13
}

impl BitmapFormat {
    pub fn new_empty(format: u8, paletteformat: u8) -> Option<BitmapFormat> {
        use BitmapFormat::*;
        match format {
            FORMAT_C4 => Some(C4(Vec::new(), PaletteC4::new_empty(paletteformat)?)),
            FORMAT_C8 => Some(C8(Vec::new(), PaletteC8::new_empty(paletteformat)?)),
            FORMAT_RGB565 => Some(RGB565(Vec::new())),
            FORMAT_A3RGB565 => Some(RGB5A3(Vec::new())),
            FORMAT_ARGB8888 => Some(RGBA8888(Vec::new())),
            FORMAT_RGB888 => Some(RGB888(Vec::new())),
            _ => None,
        }
    }

    pub fn get_format(&self) -> u8 {
        use BitmapFormat::*;
        match self {
            C4(_, _) => FORMAT_C4,
            C8(_, _) => FORMAT_C8,
            RGB565(_) => FORMAT_RGB565,
            RGB5A3(_) => FORMAT_A3RGB565,
            RGBA8888(_) => FORMAT_ARGB8888,
            RGB888(_) => FORMAT_RGB888,
        }
    }

    pub fn get_palette_format(&self) -> u8 {
        use BitmapFormat::*;
        match self {
            C4(_, ref p) => p.get_format(),
            C8(_, ref p) => p.get_format(),
            _ => 3,
        }
    }

    pub fn len(&self) -> usize {
        use BitmapFormat::*;
        match self {
            C4(ref v, _) => v.len(),
            C8(ref v, _) => v.len(),
            RGB565(ref v) => v.len(),
            RGB5A3(ref v) => v.len(),
            RGBA8888(ref v) => v.len(),
            RGB888(ref v) => v.len(),
        }
    }

    pub fn get_color(&self, index: usize) -> Option<Color> {
        use BitmapFormat::*;
        match self {
            C4(ref v, ref p) => v.get(index).map(|x| p.get_color(*x)),
            C8(ref v, ref p) => v.get(index).map(|x| p.get_color(*x)),
            RGB565(ref v) => v.get(index).map(|x| Color::from_RGB565(*x)),
            RGB5A3(ref v) => v.get(index).map(|x| Color::from_A3RGB5(*x)),
            RGBA8888(ref v) => v.get(index).map(|x| *x),
            RGB888(ref v) => v.get(index).map(|x| Color {
                r: x.0,
                g: x.1,
                b: x.2,
                a: 255,
            }),
        }
    }

    pub fn get_colors_as_vec(&self) -> Vec<Color> {
        let mut ret = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            ret.push(self.get_color(i).unwrap());
        }
        ret
    }
}

/// The BITMAP data. Contains colors for all pixels in the bitmap.
#[derive(Clone)]
pub struct Bitmap {
    data: BitmapFormat,
    alpha: AlphaLevel,
    width: u32,
    height: u32,
    flags: u8,
    unknown: u8,
}

chum_struct! {
    pub struct BitmapStruct {
        pub alpha: [enum [u8] AlphaLevel],
        pub flags: [flags [u8] {a, b, c}],
        pub unknown: [int_custom [u8] 1, 5],
    }
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
    data: &Vec<T>,
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
        return data.clone();
    }
    let mut newdata = vec![T::default(); imagewidth * imageheight];
    for (i, col) in data.iter().enumerate() {
        newdata[get_chunk_index(i, blockwidth, blockheight, imagewidth, imageheight)] = col.clone();
    }
    newdata
}

/// Take data from a linear arrangement into a blocked arangement
fn blockify<T>(
    data: &Vec<T>,
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
        return data.clone();
    }
    let mut newdata = vec![T::default(); imagewidth * imageheight];
    // for (i, col) in data.iter().enumerate() {
    //     newdata[blockify_index(i, blockwidth, blockheight, imagewidth, imageheight)] = col.clone();
    // }
    for i in 0..(imagewidth * imageheight) {
        newdata[i] =
            data[get_chunk_index(i, blockwidth, blockheight, imagewidth, imageheight)].clone();
    }
    newdata
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
    pub fn get_struct(&self) -> BitmapStruct {
        BitmapStruct {
            alpha: self.alpha,
            flags: self.flags,
            unknown: self.unknown,
        }
    }

    pub fn from_struct(data: &BitmapStruct) -> Self {
        Bitmap {
            data: BitmapFormat::RGBA8888(Vec::new()),
            alpha: data.alpha,
            width: 0,
            height: 0,
            flags: data.flags,
            unknown: data.unknown,
        }
    }

    pub fn get_data_as_vec(&self) -> Vec<Color> {
        self.data.get_colors_as_vec()
    }

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
    pub fn get_data(&self) -> &BitmapFormat {
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
        let flags: u8 = fmt.read_u8(file)?;
        let palette_format: u8 = fmt.read_u8(file)?;
        let opacity_level: u8 = fmt.read_u8(file)?;
        let _unk: u8 = fmt.read_u8(file)?;
        let filter: u8 = fmt.read_u8(file)?;
        // TODO: Handle irregular image sizes for arrange_blocks
        let data: BitmapFormat = match format {
            FORMAT_C4 => {
                let mut indices = vec![0; (width * height) as usize];
                fmt.read_u4_into(file, &mut indices)?;
                let palette = PaletteC4::read_palette(palette_format, file, fmt)?;
                let data = arrange_blocks(&indices, 8, 8, width as usize, height as usize);
                BitmapFormat::C4(data, palette)
            }
            FORMAT_C8 => {
                let mut indices = vec![0; (width * height) as usize];
                fmt.read_u8_into(file, &mut indices)?;
                let palette = PaletteC8::read_palette(palette_format, file, fmt)?;
                let data = arrange_blocks(&indices, 8, 4, width as usize, height as usize);
                BitmapFormat::C8(data, palette)
            }
            FORMAT_RGB565 => {
                let mut data = vec![0; (width * height) as usize];
                fmt.read_u16_into(file, &mut data)?;
                let data = arrange_blocks(&data, 4, 4, width as usize, height as usize);
                BitmapFormat::RGB565(data)
            }
            FORMAT_A3RGB565 => {
                let mut data = vec![0; (width * height) as usize];
                fmt.read_u16_into(file, &mut data)?;
                let data = arrange_blocks(&data, 4, 4, width as usize, height as usize);
                BitmapFormat::RGB5A3(data)
            }
            FORMAT_ARGB8888 => {
                let data = read_u32_interleaved(file, fmt, (width * height) as usize)?;
                let coldata = arrange_blocks(&data, 4, 4, width as usize, height as usize);
                BitmapFormat::RGBA8888(coldata)
            }
            FORMAT_RGB888 => {
                let mut data = vec![(0, 0, 0); (width * height) as usize];
                for i in 0..data.len() {
                    let b = fmt.read_u8(file)?;
                    let g = fmt.read_u8(file)?;
                    let r = fmt.read_u8(file)?;
                    data[i] = (r, g, b);
                }
                BitmapFormat::RGB888(data)
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
            flags,
            unknown: filter,
        })
    }

    /// Read a TMesh from data
    pub fn read_data(data: &[u8], fmt: TotemFormat) -> io::Result<Bitmap> {
        Bitmap::read_from(&mut data.as_ref(), fmt)
    }

    pub fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<()> {
        fmt.write_u32(writer, self.width)?;
        fmt.write_u32(writer, self.height)?;
        fmt.write_u32(writer, 0)?;
        fmt.write_u8(writer, self.data.get_format())?;
        fmt.write_u8(writer, self.flags)?;
        fmt.write_u8(writer, self.data.get_palette_format())?;
        fmt.write_u8(writer, self.alpha.as_u8())?;
        fmt.write_u8(writer, 0)?;
        fmt.write_u8(writer, self.unknown)?;
        match self.data {
            BitmapFormat::C4(ref v, ref p) => {
                let data = blockify(v, 8, 8, self.width as usize, self.height as usize);
                for chunk in data.chunks_exact(2) {
                    let value = (chunk[0] << 4) | chunk[1];
                    fmt.write_u8(writer, value)?;
                }
                p.write_to(writer, fmt)?;
            }
            BitmapFormat::C8(ref v, ref p) => {
                let data = blockify(v, 8, 4, self.width as usize, self.height as usize);
                for value in data.iter() {
                    fmt.write_u8(writer, *value)?;
                }
                p.write_to(writer, fmt)?;
            }
            BitmapFormat::RGB565(ref v) | BitmapFormat::RGB5A3(ref v) => {
                let data = blockify(v, 4, 4, self.width as usize, self.height as usize);
                for value in data.iter() {
                    fmt.write_u16(writer, *value)?;
                }
            }
            BitmapFormat::RGBA8888(ref v) => {
                let data = blockify(v, 4, 4, self.width as usize, self.height as usize);
                for value in data.chunks(16) {
                    let mut buf = [0u8; 64];
                    for (i, color) in value.iter().enumerate() {
                        buf[i * 2] = color.a;
                        buf[i * 2 + 1] = color.r;
                        buf[i * 2 + 32] = color.g;
                        buf[i * 2 + 33] = color.b;
                    }
                    fmt.write_bytes(writer, &buf)?;
                }
            }
            BitmapFormat::RGB888(ref v) => {
                for value in v.iter() {
                    fmt.write_u8(writer, value.2)?;
                    fmt.write_u8(writer, value.1)?;
                    fmt.write_u8(writer, value.0)?;
                }
            }
        }
        fmt.write_u32(writer, 0)?;
        Ok(())
    }

    /// Create a new bitmap with the given image data
    pub fn with_bitmap(&self, data: BitmapFormat, width: u32, height: u32) -> Bitmap {
        Bitmap {
            data,
            width,
            height,
            ..*self
        }
    }

    pub fn export_png<W>(&self, writer: &mut W) -> Result<(), Box<dyn Error>>
    where
        W: Write,
    {
        let encoder = image::png::PNGEncoder::new(writer);
        unsafe {
            let data = self.get_data_as_vec();
            let ptr = data.as_ptr() as *const u8;
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

fn palettize(data: &[Color], n: i32, width: u32, height: u32) -> (Vec<u8>, Vec<Color>) {
    let mut liq = imagequant::new();
    liq.set_max_colors(n);
    liq.set_quality(0, 100);
    let mut liq_image = liq
        .new_image(data, width as usize, height as usize, 0.0)
        .unwrap();

    let mut res = match liq.quantize(&liq_image) {
        Ok(res) => res,
        Err(err) => panic!("Failed quantization: {:?}", err),
    };

    res.set_dithering_level(1.0);

    let (palette, pixels) = res.remapped(&mut liq_image).unwrap();
    if palette.len() as i32 > n {
        panic!("Resulting palette has too many colors (this should not happen)");
    }

    (
        pixels,
        palette
            .into_iter()
            .map(|x| Color {
                r: x.r,
                g: x.g,
                b: x.b,
                a: x.a,
            })
            .collect(),
    )
}

fn read_into_palette_c4(colors: Vec<Color>, palette: &mut PaletteC4) {
    for i in 0..colors.len().min(16) {
        let col = &colors[i];
        palette.data[i] = match palette.format {
            PaletteFormat::RGB565 => col.to_RGB565() as u32,
            PaletteFormat::RGB5A3 => col.to_A3RGB5() as u32,
            PaletteFormat::RGBA8888 => col.to_RGBA8888(),
        };
    }
}

fn read_into_palette_c8(colors: Vec<Color>, palette: &mut PaletteC8) {
    for i in 0..colors.len().min(256) {
        let col = &colors[i];
        palette.data[i] = match palette.format {
            PaletteFormat::RGB565 => col.to_RGB565() as u32,
            PaletteFormat::RGB5A3 => col.to_A3RGB5() as u32,
            PaletteFormat::RGBA8888 => col.to_RGBA8888(),
        };
    }
}

pub fn compress_bitmap(data: &[Color], basis: &mut BitmapFormat, width: u32, height: u32) {
    match basis {
        BitmapFormat::RGBA8888(ref mut v) => {
            v.clear();
            v.extend(data.iter());
        }
        BitmapFormat::RGB888(ref mut v) => {
            v.clear();
            v.extend(data.iter().map(|x| (x.r, x.g, x.b)));
        }
        BitmapFormat::RGB565(ref mut v) => {
            v.clear();
            v.extend(data.iter().map(|x| x.to_RGB565()));
        }
        BitmapFormat::RGB5A3(ref mut v) => {
            v.clear();
            v.extend(data.iter().map(|x| x.to_A3RGB5()));
        }
        BitmapFormat::C4(ref mut v, ref mut palette) => {
            let (newdata, newpalette) = palettize(data, 16, width, height);
            v.clear();
            v.extend(newdata.into_iter());
            read_into_palette_c4(newpalette, palette);
        }
        BitmapFormat::C8(ref mut v, ref mut palette) => {
            let (newdata, newpalette) = palettize(data, 256, width, height);
            v.clear();
            v.extend(newdata.into_iter());
            read_into_palette_c8(newpalette, palette);
        }
    }
}

pub fn import_bitmap<R>(
    reader: &mut R,
    format: image::ImageFormat,
) -> Result<(Vec<Color>, u32, u32), Box<dyn Error>>
where
    R: Read + BufRead + Seek,
{
    use image::GenericImageView;
    let mut imgreader = image::io::Reader::new(reader);
    imgreader.set_format(format);
    let image = imgreader.decode()?;
    let image = if image.width() % 8 != 0 || image.height() % 8 != 0 {
        let width = util::round_up(image.width() as usize, 8) as u32;
        let height = util::round_up(image.height() as usize, 8) as u32;
        image.resize_exact(width, height, image::imageops::FilterType::CatmullRom)
    } else {
        image
    };
    let image = image.to_rgba();
    let (width, height) = image.dimensions();
    let mut buf = Vec::with_capacity(width as usize * height as usize);
    for iy in 0..height {
        for ix in 0..width {
            let pixel = image.get_pixel(ix, iy);
            buf.push(Color {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
                a: pixel[3],
            });
        }
    }
    Ok((buf, width, height))
}
