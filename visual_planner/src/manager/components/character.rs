use types::*;

pub struct CharacterModel(Arc<CharacterModelInternal>);



pub struct CharacterModelInternal {
   id: CharacterID, 
   name: Mutex<String>,
   states: Mutex<Vec<CharacterState>>
}


pub struct CharacterState(Arc<CharacterStateIternal>);

pub struct CharacterStateInternal {
    name: Mutex<String>,
    image: Mutex<Path>
}