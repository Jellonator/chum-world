use gdnative::prelude::*;
pub mod bitmap;
pub mod collisionvol;
pub mod light;
pub mod lod;
pub mod material;
pub mod materialanim;
pub mod mesh;
pub mod node;
pub mod omni;
pub mod rotshape;
pub mod skin;
pub mod sound;

pub fn init(handle: InitHandle) {
    handle.add_class::<bitmap::BitmapView>();
    handle.add_class::<collisionvol::CollisionVolView>();
    handle.add_class::<light::LightView>();
    handle.add_class::<lod::LodView>();
    handle.add_class::<material::MaterialView>();
    handle.add_class::<materialanim::MaterialAnimView>();
    handle.add_class::<mesh::MeshView>();
    handle.add_class::<node::NodeView>();
    handle.add_class::<omni::OmniView>();
    handle.add_class::<sound::SoundView>();
    handle.add_class::<rotshape::RotShapeView>();
    handle.add_class::<skin::SkinView>();
}
