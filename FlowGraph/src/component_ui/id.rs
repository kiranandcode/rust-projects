use types::*;

use std::fmt::{Display, Formatter};
use std::error::Error;


/// - - - - - - - - - - - - - - - - - - - - -
///                Graph
/// - - - - - - - - - - - - - - - - - - - - -
pub type IndirectID = usize;
pub type TimeStamp = usize;
pub type RawID = usize;
#[derive(Debug,Default,PartialEq,Clone,Copy, PartialOrd, Ord, Eq)]
pub struct ID(IndirectID, TimeStamp);

pub struct IDManager {
    map: Vec<(TimeStamp, Option<RawID>)>,
    next_empty_index: Option<usize>,
    empty_spaces: usize
}
#[derive(Debug)]
pub enum IDError {IDOutDated, IDOutOfRange, IDDeleted}
impl Display for IDError {fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {write!(f, "{:?}", self)}}
impl Error for IDError {}

impl Default for IDManager {
    fn default() -> Self {
        IDManager {
            map: Vec::new(),
            next_empty_index: None,
            empty_spaces: 0
        }
    }
}
impl IDManager {
    pub fn get(&self, id: ID) -> Result<RawID, IDError> {
        if self.map.len() < id.0  {
            Err(IDError::IDOutOfRange)
        } else {
            let (timestamp, maybe_id) = self.map[id.0];
            if timestamp != id.1 {
                Err(IDError::IDOutDated)
            } else {
                Ok(maybe_id.unwrap())
            }
        }
    }

    // identifies whether an id is valid or not
    pub fn valid(&self, id: ID) -> bool {
        if self.map.len() < id.0 {
            false
        } else {
            let (timestamp, maybe_id) = self.map[id.0];
            if timestamp != id.1 {
                false
            } else {
                true
            }
        }
    }

    pub fn new(&mut self, pos: RawID) -> ID {
        // if we have a cached next index to insert into, insert into it.
        if let Some(index) = self.next_empty_index.take() {
            let ts = self.map[index].0;
            self.map[index].1 = Some(pos);
            self.empty_spaces -= 1;
            ID(index, ts)
        } else {
            // otherwise, check for any empty spaces
            if self.empty_spaces > 0 {
                let mut index = 0;
                while index < self.map.len() {
                    if self.map[index].1.is_none() {
                        let ts = self.map[index].0;
                        self.map[index].1 = Some(pos);
                        self.empty_spaces -= 1;
                        return ID(index, ts);
                    }
                }
            }
            // otherwise just insert
            self.map.push((0, Some(pos)));
            let (ts, _) = (self.map[self.map.len() - 1]);
            ID(self.map.len() - 1, ts)
        }
    }


    /// removes a binding for an id, leaving an empty space
    /// used when the object to be removed is the last object, so no swaps occur
    /// Note: not intended to be called directly, but by the ContentInner
    pub fn remove(&mut self, id: ID) -> Result<(), IDError> {
        if self.map.len() < id.0  {
            Err(IDError::IDOutOfRange)
        } else {
            // grab the position of the mapping
            let (ref mut timestamp, ref mut maybe_id) = self.map[id.0];
            // check that the timestamps are correct
            if *timestamp != id.1 {
                return Err(IDError::IDOutDated);
            }
            // remove the mapping
            if let Some(old_raw_id) = maybe_id.take() {
                // update the timestamp of the mapping so that the id is invalidated
                *timestamp += 1;

                // also add a reference to this index as a potential empty space
                if self.next_empty_index.is_none() {
                    self.next_empty_index = Some(id.0);
                }
                // inform the structure of the new empty space
                self.empty_spaces += 1;

                // done
                Ok(())
            } else {
                // unlikely case, timestamps match, but index is empty
                Err(IDError::IDDeleted)
            }
        }
    }


    /// removes a binding for an id, and updates a replacement to point to the removed items location
    /// should be used in conjunction with swap_remove, when the index isn't the last one
    pub fn swap_remove(&mut self, id: ID, replacement: ID) -> Result<(), IDError> {

        // We have a list of id refs
        //
        //  [ ] -> [1]   (* i.e id no. 0 points to index 1*)
        //  [ ] -> [2]
        //  [ ] -> [0]
        //  [ ] -> [3]
        //
        // which corresponds to a list of objects
        //
        // [0] -> Obj0
        // [1] -> Obj1 <--
        // [2] -> Obj2    |
        // [3] -> Obj3  --
        //
        // now, we have just swap removed one of the objects - let's say 1, by 3
        // thus removing object 1
        //
        // [0] -> Obj0
        // [1] -> Obj3
        // [2] -> Obj2
        //
        // but the problem is, now all our ids to object 3,
        // (which have an id index of 3), now point to 1 off the end of the
        // array.
        // to fix this, we need to update our id_refs to
        //
        //  [ ] -> [ ]   (* the id that used to point to obj 1 is dead *)
        //  [ ] -> [2]
        //  [ ] -> [0]
        //  [ ] -> [1]   (* the old id now points to obj 3 again *)

        // start with some sanity checks
        if self.map.len() < id.0  {
            // check that the base index is within range
            Err(IDError::IDOutOfRange)
        } else if self.map.len() < replacement.0 {
            // check that the replacement index is within range
            Err(IDError::IDOutOfRange)
        } else {

            // sanity check for the replacement id
            if self.map[replacement.0].0 != replacement.1 {
                return Err(IDError::IDOutDated);
            }

            // grab the raw id, the removed object used to point to
            // note, this will now point to the replacement object
            // (in our example above, this would be index 1, with the
            // replacement being obj3 )
            let maybe_old_id = {
                let (ref mut timestamp, ref mut maybe_id) = self.map[id.0];

                // check that the timestamps match
                if *timestamp != id.1 {
                    return Err(IDError::IDOutDated);
                }
                // here we also invalidate the old reference
                if let Some(old_id) = maybe_id.take() {
                    // this means the old_id exists, so increment the timestamp
                    // as we will be able to remove successfully
                    *timestamp += 1;
                    Some(old_id)
                } else {
                    None
                }
            };


            if let Some(old_raw_id) =  maybe_old_id {
                // simple sanity check, this function should only be called given one valid id to be removed, and one valid id to be removed
                assert!(self.map[replacement.0].1.is_some());

                // update the replacement index to point to raw_id
                // in our example this would be updating the binding of id 3,
                // to point to index 1, as this is where object 3 now lives
                self.map[replacement.0].1 = Some(old_raw_id);

                // no need to increment the timestamp to indivalidate the
                // remove id, as we did it earlier

                // register this index as a potential empty space
                if self.next_empty_index.is_none() {
                    self.next_empty_index = Some(id.0);
                }

                // update the number of empty spaces
                self.empty_spaces += 1;

                Ok(())
            } else {
                // unlikely case, timestamps match, but old value has been removed
                return Err(IDError::IDDeleted);
            }
        }
    }
}
