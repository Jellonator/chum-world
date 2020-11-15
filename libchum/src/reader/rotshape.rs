use crate::common::*;

chum_struct_generate_readwrite! {
// chum_struct! {
    /// Rotation shape
    pub struct RotShape {
        pub transform: [struct TransformationHeader],
        pub junk1: [ignore [u32]],
        pub offset: [Vector3],
        pub junk2: [ignore [u32]],
        pub unk7: [reference],
        pub junk3: [ignore [u32]],
        pub size: [fixed array [Vector3] 2],
        pub junk4: [ignore [u32]],
        pub texcoords: [fixed array [Vector2] 4],
        pub junk5: [ignore [u32]],
        pub materialanim_id: [reference MATERIALANIM],
        pub billboard_mode: [enum [u16] BillBoardMode],
    }
}

chum_enum! {
    #[derive(Copy, Clone, Debug)]
    pub enum BillBoardMode {
        YAxis,
        Full,
    }
}
