use crate::common::*;

chum_binary! {
    pub struct Spline {
        pub transform: [struct THeader],
        pub item_type: [ignore [u16] ITEM_TYPE_SPLINE],
        pub item_subtype: [ignore [u16] 2u16],
        pub vertices: [dynamic array [u32] [Vector3] Vector3::default()],
        pub sections: [dynamic array [u32] [struct SplineSection] SplineSection::default()],
        pub unk4: [fixed array [f32] 4],
        pub length: [f32],
    }
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
}

chum_binary! {
    #[derive(Default)]
    pub struct SplineSection {
        pub p1: [u32],
        pub p2: [u32],
        pub p1_t: [u32],
        pub p2_t: [u32],
        pub unk: [u32],
        pub section_length: [f32],
        pub subsections: [fixed array [struct SplineSubsection] 8],
    }
}

chum_binary! {
    #[derive(Default)]
    pub struct SplineSubsection {
        pub point1: [Vector3],
        pub point2: [Vector3],
        pub subsection_length: [f32],
    }
}
