use crate::common::*;
use crate::format::TotemFormat;
use std::io::{self, Read};

pub struct Spline {
    pub transform: TransformationHeader,
    pub vertices: Vec<Vector3>,
    pub sections: Vec<SplineSection>,
    pub unk4: [f32; 4],
    pub length: f32,
}

impl Spline {
    pub fn get_vertices_as_vec(&self) -> Vec<Vector3> {
        let mut v = Vec::with_capacity(self.sections.len() * 8 + 1);
        if self.sections.len() > 0 {
            v.push(self.sections[0].subsections[0].point1);
            for section in self.sections.iter() {
                for subsection in section.subsections.iter() {
                    v.push(subsection.point2);
                }
            }
        }
        v
    }

    pub fn get_section_stops_as_vec(&self) -> Vec<Vector3> {
        let mut v = Vec::with_capacity(self.sections.len() * 8 + 1);
        if self.sections.len() > 0 {
            v.push(self.sections[0].subsections[0].point1);
            for section in self.sections.iter() {
                v.push(section.subsections[7].point2);
            }
        }
        v
    }

    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<Spline> {
        Ok(Spline {
            transform: TransformationHeader::read_from(file, fmt)?,
            vertices: {
                let num = fmt.read_u32(file)?;
                let mut vec = Vec::with_capacity(num as usize);
                for _ in 0..num {
                    vec.push(read_vec3(file, fmt)?);
                }
                vec
            },
            sections: {
                let num = fmt.read_u32(file)?;
                let mut vec = Vec::with_capacity(num as usize);
                for _ in 0..num {
                    vec.push(SplineSection::read_from(file, fmt)?);
                }
                vec
            },
            unk4: [
                fmt.read_f32(file)?,
                fmt.read_f32(file)?,
                fmt.read_f32(file)?,
                fmt.read_f32(file)?,
            ],
            length: fmt.read_f32(file)?,
        })
    }
}

pub struct SplineSection {
    pub p1: u32,
    pub p2: u32,
    pub p1_t: u32,
    pub p2_t: u32,
    pub unk: u32,
    pub section_length: f32,
    pub subsections: [SplineSubsection; 8],
}

impl SplineSection {
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<SplineSection> {
        Ok(SplineSection {
            p1: fmt.read_u32(file)?,
            p2: fmt.read_u32(file)?,
            p1_t: fmt.read_u32(file)?,
            p2_t: fmt.read_u32(file)?,
            unk: fmt.read_u32(file)?,
            section_length: fmt.read_f32(file)?,
            subsections: [
                SplineSubsection::read_from(file, fmt)?,
                SplineSubsection::read_from(file, fmt)?,
                SplineSubsection::read_from(file, fmt)?,
                SplineSubsection::read_from(file, fmt)?,
                SplineSubsection::read_from(file, fmt)?,
                SplineSubsection::read_from(file, fmt)?,
                SplineSubsection::read_from(file, fmt)?,
                SplineSubsection::read_from(file, fmt)?,
            ],
        })
    }
}

pub struct SplineSubsection {
    pub point1: Vector3,
    pub point2: Vector3,
    pub subsection_length: f32,
}

impl SplineSubsection {
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<SplineSubsection> {
        Ok(SplineSubsection {
            point1: read_vec3(file, fmt)?,
            point2: read_vec3(file, fmt)?,
            subsection_length: fmt.read_f32(file)?,
        })
    }
}
