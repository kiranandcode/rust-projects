use ::std::collections::HashMap;
use ::std::fmt::{
    Display,
    Formatter,
};
use ::std::fmt;



pub struct NgramFrequency {
    frequency: HashMap<String,f64>,
    slice_size: usize,
    total: u32,
    floor: f64
}


impl NgramFrequency {
    pub fn generate_from(text : &String, slice_size: usize) -> NgramFrequency {
        let max = text.len() - slice_size;
        let mut freq : HashMap<String,f64> = HashMap::new();

        for i in 0..max {
            let slice = text.as_str()[i..i+slice_size].to_string();
            let fr = freq.entry(slice).or_insert(0.0);
            *fr += 1.0;
        }
        let mut total : u32 = 0;

        for (key,count) in freq.iter() {
            total = total + *count as u32;
        }

        let mut floor = 0.01 / (total as f64);
        floor.log(10.0);

        for (key, mut count) in (&mut freq).into_iter() {
            if *count == 1.0 {
                *count = floor;
            } else {
                *count = *count / (total as f64);
                count.log(10.0);
            }
        }

        NgramFrequency {
            frequency : freq,
            slice_size,
            total,
            floor
        }
    }


    pub fn score_text(&self, text : &String) -> f64 {
        let max = text.len() - self.slice_size;
        let mut score : f64 = 0.0;
        for i in 0..max {
            let slice = text.as_str()[i..i+self.slice_size].to_string();
            match self.frequency.get(&slice) {
                Some(count) => { score = score +  count;}
                None => { score = score + self.floor; }
            }
        }

        score
    }
}


impl Display for NgramFrequency {
    
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut count = 0;
        let mut vec : Vec<(&String,&f64)> = self.frequency.iter().collect();
        vec.sort_by(|a, b| b.1.partial_cmp(a.1).expect("Couldn't compare values"));
        write!(f, "{}", vec.len());
        for &(key,val) in vec.iter() {
            write!(f, "{}: {}, ", key, *val);
            count = count + 1;
            if count > 10 {
                break;
            }
        }
        write!(f, "\n")
    }
}
