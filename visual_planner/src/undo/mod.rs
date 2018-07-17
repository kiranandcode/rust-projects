pub enum Modification {
    UpdateValues,
    New,
    Deleted
}


pub trait Modifiable {
    fn update_state(&mut self, other: &Self) -> Modification;
}