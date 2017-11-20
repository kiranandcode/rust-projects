use ::std::collections::HashMap;
use ::std::fmt::{
    Display,
    Formatter,
};
use ::std::fmt;



pub struct NgramFrequency {
    frequency: HashMap<String,u16>,
    slice_size: usize
}


impl NgramFrequency {
    pub fn generate_from(text : &String, slice_size: usize) -> NgramFrequency {
        let max = text.len() - slice_size;
        let mut freq : HashMap<String,u16> = HashMap::new();

        for i in 0..max {
            let slice = text.as_str()[i..i+slice_size].to_string();
            let fr = freq.entry(slice).or_insert(0);
            *fr += 1;
        }

        NgramFrequency {
            frequency : freq,
            slice_size
        }
    }
}


impl Display for NgramFrequency {
    
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut vec : Vec<(&String,&u16)> = self.frequency.iter().collect();
        vec.sort_by(|a, b| b.1.cmp(a.1));
        write!(f, "{}", vec.len());
        for &(key,val) in vec.iter() {
            write!(f, "{}: {}, ", key, *val);
        }
        write!(f, "\n")
    }
}
