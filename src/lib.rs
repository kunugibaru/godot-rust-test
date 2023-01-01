use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloWorld;

#[methods]
impl HelloWorld {
    fn new(_base: &Node) -> Self {
        HelloWorld
    }

    #[method]
    fn _ready(&self, #[base] _base: &Node) {
        godot_print!("Hello, world.");
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<HelloWorld>();
}

godot_init!(init);