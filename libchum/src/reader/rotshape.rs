use crate::common::*;

chum_struct_generate_readwrite! {
// chum_struct! {
    /// Rotation shape
    pub struct RotShape {
        pub transform: [struct TransformationHeader],
        pub offset: [Vector3],
        pub unk7: [reference],
        pub size: [fixed array [Vector3] 2],
        pub texcoords: [fixed array [Vector2] 4],
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
