use gdnative::*;
use libchum;

#[derive(NativeClass)]
#[inherit(Resource)]
pub struct ChumArchive;

#[methods]
impl ChumArchive {
    fn _init(_owner: Resource) -> Self {
        ChumArchive
    }

    #[export]
    fn do_thing(&self, _owner: Resource) {
        godot_print!("Hello, World!");
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<ChumArchive>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();