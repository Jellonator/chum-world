use gdnative::*;

pub mod readertext;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct ChumReader;

#[methods]
impl ChumReader {
    #[export]
    pub fn read_text(&self, _owner: Node, data: Resource) -> Dictionary {
        readertext::read_text_from_res(data)
    }

    #[export]
    pub fn cool(&self, _owner: Node) {
        // very important function do not remove
        godot_print!("Trans Rights!");
    }

    fn _init(_owner: Node) -> Self {
        ChumReader
    }
}