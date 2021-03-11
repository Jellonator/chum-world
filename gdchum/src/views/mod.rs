use gdnative::prelude::*;
pub mod node;
pub mod sound;
pub mod bitmap;
pub mod collisionvol;
pub mod light;
pub mod material;
pub mod materialanim;

pub fn init(handle: InitHandle) {
    handle.add_class::<node::NodeView>();
    handle.add_class::<sound::SoundView>();
    handle.add_class::<bitmap::BitmapView>();
    handle.add_class::<collisionvol::CollisionVolView>();
    handle.add_class::<light::LightView>();
    handle.add_class::<material::MaterialView>();
    handle.add_class::<materialanim::MaterialAnimView>();
}
