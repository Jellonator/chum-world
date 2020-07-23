use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

/// The format of the Totem archive.
/// There is one main difference between the Gamecube builds and Playstation 2 builds:
/// The Gamecube is big-endian, while the Playstation 2 is little-endian.
#[derive(Copy, Clone, Debug)]
pub enum TotemFormat {
    NGC, // Gamecube
    PS2, // Playstation2
}

impl TotemFormat {
    // writing functions
    pub fn write_u32<W: Write>(&self, writer: &mut W, value: u32) -> io::Result<()> {
        match self {
            TotemFormat::NGC => writer.write_u32::<BigEndian>(value),
            TotemFormat::PS2 => writer.write_u32::<LittleEndian>(value),
        }
    }
    pub fn write_i32<W: Write>(&self, writer: &mut W, value: i32) -> io::Result<()> {
        match self {
            TotemFormat::NGC => writer.write_i32::<BigEndian>(value),
            TotemFormat::PS2 => writer.write_i32::<LittleEndian>(value),
        }
    }
    pub fn write_u24<W: Write>(&self, writer: &mut W, value: u32) -> io::Result<()> {
        match self {
            TotemFormat::NGC => writer.write_u24::<BigEndian>(value),
            TotemFormat::PS2 => writer.write_u24::<LittleEndian>(value),
        }
    }
    pub fn write_i24<W: Write>(&self, writer: &mut W, value: i32) -> io::Result<()> {
        match self {
            TotemFormat::NGC => writer.write_i24::<BigEndian>(value),
            TotemFormat::PS2 => writer.write_i24::<LittleEndian>(value),
        }
    }
    pub fn write_u16<W: Write>(&self, writer: &mut W, value: u16) -> io::Result<()> {
        match self {
            TotemFormat::NGC => writer.write_u16::<BigEndian>(value),
            TotemFormat::PS2 => writer.write_u16::<LittleEndian>(value),
        }
    }
    pub fn write_i16<W: Write>(&self, writer: &mut W, value: i16) -> io::Result<()> {
        match self {
            TotemFormat::NGC => writer.write_i16::<BigEndian>(value),
            TotemFormat::PS2 => writer.write_i16::<LittleEndian>(value),
        }
    }
    pub fn write_u8<W: Write>(&self, writer: &mut W, value: u8) -> io::Result<()> {
        writer.write_u8(value)
    }
    pub fn write_i8<W: Write>(&self, writer: &mut W, value: i8) -> io::Result<()> {
        writer.write_i8(value)
    }
    pub fn write_f32<W: Write>(&self, writer: &mut W, value: f32) -> io::Result<()> {
        match self {
            TotemFormat::NGC => writer.write_f32::<BigEndian>(value),
            TotemFormat::PS2 => writer.write_f32::<LittleEndian>(value),
        }
    }
    pub fn write_bytes<W: Write>(&self, writer: &mut W, buf: &[u8]) -> io::Result<()> {
        writer.write_all(buf)
    }
    // reading functions
    pub fn read_u32<R: Read>(&self, reader: &mut R) -> io::Result<u32> {
        match self {
            TotemFormat::NGC => reader.read_u32::<BigEndian>(),
            TotemFormat::PS2 => reader.read_u32::<LittleEndian>(),
        }
    }
    pub fn read_i32<R: Read>(&self, reader: &mut R) -> io::Result<i32> {
        match self {
            TotemFormat::NGC => reader.read_i32::<BigEndian>(),
            TotemFormat::PS2 => reader.read_i32::<LittleEndian>(),
        }
    }
    pub fn read_u24<R: Read>(&self, reader: &mut R) -> io::Result<u32> {
        match self {
            TotemFormat::NGC => reader.read_u24::<BigEndian>(),
            TotemFormat::PS2 => reader.read_u24::<LittleEndian>(),
        }
    }
    pub fn read_i24<R: Read>(&self, reader: &mut R) -> io::Result<i32> {
        match self {
            TotemFormat::NGC => reader.read_i24::<BigEndian>(),
            TotemFormat::PS2 => reader.read_i24::<LittleEndian>(),
        }
    }
    pub fn read_u16<R: Read>(&self, reader: &mut R) -> io::Result<u16> {
        match self {
            TotemFormat::NGC => reader.read_u16::<BigEndian>(),
            TotemFormat::PS2 => reader.read_u16::<LittleEndian>(),
        }
    }
    pub fn read_i16<R: Read>(&self, reader: &mut R) -> io::Result<i16> {
        match self {
            TotemFormat::NGC => reader.read_i16::<BigEndian>(),
            TotemFormat::PS2 => reader.read_i16::<LittleEndian>(),
        }
    }
    pub fn read_u8<R: Read>(&self, reader: &mut R) -> io::Result<u8> {
        reader.read_u8()
    }
    pub fn read_i8<R: Read>(&self, reader: &mut R) -> io::Result<i8> {
        reader.read_i8()
    }
    pub fn read_f32<R: Read>(&self, reader: &mut R) -> io::Result<f32> {
        match self {
            TotemFormat::NGC => reader.read_f32::<BigEndian>(),
            TotemFormat::PS2 => reader.read_f32::<LittleEndian>(),
        }
    }
    pub fn read_exact<R: Read>(&self, reader: &mut R, buf: &mut [u8]) -> io::Result<()> {
        reader.read_exact(buf)
    }
    pub fn read_to_end<R: Read>(&self, reader: &mut R) -> io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(buf)
    }
    pub fn skip_n_bytes<R: Read>(&self, reader: &mut R, n: u64) -> io::Result<()> {
        io::copy(&mut reader.take(n), &mut io::sink())?;
        Ok(())
    }
    // Read into functions
    pub fn read_u4_into<R: Read>(&self, reader: &mut R, dst: &mut [u8]) -> io::Result<()> {
        for i in 0..(dst.len() / 2) {
            let value = self.read_u8(reader)?;
            dst[i * 2] = value >> 4;
            dst[i * 2 + 1] = value & 0x0F;
        }
        if dst.len() % 2 == 1 {
            let value = self.read_u8(reader)?;
            dst[dst.len() - 1] = value >> 4;
        }
        Ok(())
    }
    pub fn read_u24_into<R: Read>(&self, reader: &mut R, dst: &mut [u32]) -> io::Result<()> {
        for i in 0..dst.len() {
            dst[i] = self.read_u24(reader)?;
        }
        Ok(())
    }
    pub fn read_u32_into<R: Read>(&self, reader: &mut R, dst: &mut [u32]) -> io::Result<()> {
        match self {
            TotemFormat::NGC => reader.read_u32_into::<BigEndian>(dst),
            TotemFormat::PS2 => reader.read_u32_into::<LittleEndian>(dst),
        }
    }
    pub fn read_i32_into<R: Read>(&self, reader: &mut R, dst: &mut [i32]) -> io::Result<()> {
        match self {
            TotemFormat::NGC => reader.read_i32_into::<BigEndian>(dst),
            TotemFormat::PS2 => reader.read_i32_into::<LittleEndian>(dst),
        }
    }
    pub fn read_u16_into<R: Read>(&self, reader: &mut R, dst: &mut [u16]) -> io::Result<()> {
        match self {
            TotemFormat::NGC => reader.read_u16_into::<BigEndian>(dst),
            TotemFormat::PS2 => reader.read_u16_into::<LittleEndian>(dst),
        }
    }
    pub fn read_i16_into<R: Read>(&self, reader: &mut R, dst: &mut [i16]) -> io::Result<()> {
        match self {
            TotemFormat::NGC => reader.read_i16_into::<BigEndian>(dst),
            TotemFormat::PS2 => reader.read_i16_into::<LittleEndian>(dst),
        }
    }
    pub fn read_f32_into<R: Read>(&self, reader: &mut R, dst: &mut [f32]) -> io::Result<()> {
        match self {
            TotemFormat::NGC => reader.read_f32_into::<BigEndian>(dst),
            TotemFormat::PS2 => reader.read_f32_into::<LittleEndian>(dst),
        }
    }
    pub fn read_u8_into<R: Read>(&self, reader: &mut R, dst: &mut [u8]) -> io::Result<()> {
        reader.read_exact(dst)
    }
    pub fn read_i8_into<R: Read>(&self, reader: &mut R, dst: &mut [i8]) -> io::Result<()> {
        reader.read_i8_into(dst)
    }
}
