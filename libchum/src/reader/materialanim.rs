use crate::common::*;
use crate::format::TotemFormat;
use std::io::{self, Read};

#[derive(Clone, Copy)]
pub enum Interpolation {
    Discrete,
    Linear,
    Unknown,
    Invalid,
}

pub struct TrackFrame<T> {
    pub frame: u16,
    pub data: T,
}

pub struct Track<T> {
    pub interp: Interpolation,
    pub frames: Vec<TrackFrame<T>>,
}

impl<T> Track<T> {
    pub fn find_frame(&self, frame: u16) -> Option<(&T, &T, f32)> {
        if self.frames.len() == 0 {
            None
        } else if self.frames.len() == 1 {
            Some((&self.frames[0].data, &self.frames[1].data, 0.0))
        } else {
            // Todo: Handle interpolation
            for i in 1..self.frames.len() {
                if self.frames[i].frame >= frame {
                    let prev = &self.frames[i - 1];
                    let curr = &self.frames[i];
                    let t = (frame as f32 - prev.frame as f32)
                        / (curr.frame as f32 - prev.frame as f32);
                    return Some((&prev.data, &curr.data, t));
                }
            }
            Some((
                &self.frames.last().unwrap().data,
                &self.frames.last().unwrap().data,
                0.0,
            ))
        }
    }
    pub fn find_frame_index(&self, frame: u16) -> Option<usize> {
        if self.frames.len() == 0 {
            None
        } else if self.frames.len() == 1 {
            Some(0)
        } else {
            // Todo: Handle interpolation
            for i in 1..self.frames.len() {
                if self.frames[i].frame >= frame {
                    return Some(i - 1);
                }
            }
            Some(self.frames.len() - 1)
        }
    }

    pub fn len(&self) -> usize {
        self.frames.len()
    }
}

pub struct MaterialAnimation {
    pub length: f32,
    pub material_id: i32,
    pub track_texture: Track<i32>,
    pub track_scroll: Track<Vector2>,
    pub track_stretch: Track<Vector2>,
    pub track_rotation: Track<f32>,
    pub track_color: Track<[f32; 3]>,
    // pub track_unk:   Track<Vector3>,
    pub track_alpha: Track<f32>,
    // pub track_unk1:  Track<[u8; 4]>,
    // pub track_unk2:  Track<[u8; 4]>,
    // pub track_unk3:  Track<[u8; 4]>,
}

fn u16_to_interp(value: u16) -> Interpolation {
    match value {
        1 => Interpolation::Discrete,
        2 => Interpolation::Linear,
        3 => Interpolation::Unknown,
        _ => Interpolation::Invalid,
    }
}

fn read_track<T, F, R>(file: &mut R, fmt: TotemFormat, func: F) -> io::Result<Track<T>>
where
    F: Fn(&mut R, TotemFormat) -> io::Result<T>,
    R: Read,
{
    let interp = u16_to_interp(fmt.read_u16(file)?);
    let mut frames = Vec::new();
    for _ in 0..fmt.read_u32(file)? {
        let frame = fmt.read_u16(file)?;
        fmt.skip_n_bytes(file, 2)?;
        let data = func(file, fmt)?;
        frames.push(TrackFrame::<T> { frame, data });
    }
    Ok(Track { interp, frames })
}

fn read_texture_track<T, F, R>(file: &mut R, fmt: TotemFormat, func: F) -> io::Result<Track<T>>
where
    F: Fn(&mut R, TotemFormat) -> io::Result<T>,
    R: Read,
{
    let interp = u16_to_interp(fmt.read_u16(file)?);
    let mut frames = Vec::new();
    for _ in 0..fmt.read_u32(file)? {
        let data = func(file, fmt)?;
        let frame = fmt.read_u16(file)?;
        frames.push(TrackFrame::<T> { frame, data });
    }
    Ok(Track { interp, frames })
}

impl MaterialAnimation {
    /// Read a MaterialAnimation from a file
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<MaterialAnimation> {
        fmt.skip_n_bytes(file, 1)?;
        let length = fmt.read_f32(file)?;
        let track_texture = read_texture_track(file, fmt, |file, fmt| fmt.read_i32(file))?;
        let track_scroll = read_track(file, fmt, |file, fmt| {
            Ok(Vector2 {
                x: fmt.read_f32(file)?,
                y: fmt.read_f32(file)?,
            })
        })?;
        let track_stretch = read_track(file, fmt, |file, fmt| {
            Ok(Vector2 {
                x: fmt.read_f32(file)?,
                y: fmt.read_f32(file)?,
            })
        })?;
        let track_rotation = read_track(file, fmt, |file, fmt| fmt.read_f32(file))?;
        let track_color = read_track(file, fmt, |file, fmt| {
            let mut data = [0.0; 3];
            fmt.read_f32_into(file, &mut data)?;
            Ok(data)
        })?;
        let _track_unknown = read_track(file, fmt, |file, fmt| {
            Ok(Vector3 {
                x: fmt.read_f32(file)?,
                y: fmt.read_f32(file)?,
                z: fmt.read_f32(file)?,
            })
        })?;
        let track_alpha = read_track(file, fmt, |file, fmt| fmt.read_f32(file))?;
        let _track_unk1 = read_track(file, fmt, |file, fmt| fmt.read_u32(file))?;
        let _track_unk2 = read_track(file, fmt, |file, fmt| fmt.read_u32(file))?;
        let _track_unk3 = read_track(file, fmt, |file, fmt| fmt.read_u32(file))?;
        let material_id = fmt.read_i32(file)?;
        Ok(MaterialAnimation {
            length,
            track_texture,
            track_scroll,
            track_stretch,
            track_rotation,
            track_color,
            track_alpha,
            material_id,
        })
    }

    /// Read a TMesh from data
    pub fn read_data(data: &[u8], fmt: TotemFormat) -> io::Result<MaterialAnimation> {
        MaterialAnimation::read_from(&mut data.as_ref(), fmt)
    }
}
