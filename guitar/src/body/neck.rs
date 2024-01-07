pub mod strings;

#[derive(Debug)]
pub struct Neck {
    pub length: u32,
    pub strings: strings::Strings,
}
