use std::sync::Mutex;

lazy_static! {
    pub static ref DEFAULT: Mutex<Descriptor> = Mutex::new(Descriptor{});
}

pub struct Descriptor {

}
