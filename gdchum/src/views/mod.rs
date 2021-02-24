use gdnative::prelude::*;
pub mod node;
pub mod sound;

pub fn init(handle: InitHandle) {
    handle.add_class::<node::NodeView>();
    handle.add_class::<sound::SoundView>();
}
