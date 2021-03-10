use gdnative::prelude::*;
pub mod node;
pub mod sound;
pub mod bitmap;
pub mod collisionvol;

pub fn init(handle: InitHandle) {
    handle.add_class::<node::NodeView>();
    handle.add_class::<sound::SoundView>();
    handle.add_class::<bitmap::BitmapView>();
    handle.add_class::<collisionvol::CollisionVolView>();
}
