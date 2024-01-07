mod body;
mod internals;

use body::neck::strings;
pub fn create_strings(amount: u32, gauge: Vec<u32>) -> strings::Strings {
    strings::Strings::new(amount, gauge)
}

use body::neck;
pub fn create_neck(length: u32, strings: strings::Strings) -> neck::Neck {
    neck::Neck { length, strings }
}

pub fn create_body(neck: neck::Neck) -> body::Body {
    body::Body { neck }
}

#[derive(Debug)]
pub struct Guitar {
    body: body::Body,
    internals: ()
}

impl Guitar {
    pub fn new(body: body::Body) -> Guitar {
        Guitar { body, internals: () }
    }
}
