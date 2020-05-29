use crate::format::TotemFormat;
use std::io::{self, Read};

const FORMAT_C4: u8 = 1;
const FORMAT_C8: u8 = 2;
const FORMAT_RGB565: u8 = 8;
const FORMAT_A3RGB565: u8 = 10;
const FORMAT_ARGB8888: u8 = 12;
const FORMAT_RGB888: u8 = 13;

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

#[derive(Clone, Copy, Debug)]
pub enum AlphaLevel {
    Opaque, // alpha is always 255
    Bit,    // alpha is always either 0 or 255
    Blend,  // alpha can be any value
}

#[derive(Clone, Debug)]
pub struct Bitmap {
    data: Vec<Color>,
    alpha: AlphaLevel,
    width: u32,
    height: u32,
}

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
    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_data(&self) -> &[Color] {
        &self.data
    }

    pub fn get_alpha_level(&self) -> AlphaLevel {
        self.alpha
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
