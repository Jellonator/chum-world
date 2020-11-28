use gdnative::prelude::*;
pub mod node;

pub fn init(handle: InitHandle) {
    handle.add_class::<node::NodeView>();
}
