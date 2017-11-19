extern crate rand;

use self::rand::Rng;
use ::std::fmt::{
    Display,
    Formatter,
};
use ::std::fmt;

pub struct SubstitutionCipher {
    mapping: [u8; 26]
}

impl SubstitutionCipher {
    pub fn new() -> SubstitutionCipher {
       let mut chars = [ 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, ];

       rand::thread_rng().shuffle(&mut chars);

       SubstitutionCipher {
        mapping: chars
       }
    }
}

impl Display for SubstitutionCipher {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (index, character) in (0..26).zip((b'a'..b'z'+1)) {
           write!(f, "{}: {}\n", character as char, self.mapping[index] as char);
        }
        write!(f, "\n")
    }
}
