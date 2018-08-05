
use types::*;
use color::*;
use drawing_context::*;
use component_renderer::*;

use std::ops::{IndexMut, Index};
use std::fmt::{Display, Formatter};
use std::error::Error;
use std::any::Any;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref};
use std::convert::TryFrom;

use gtk::{Window, WindowExt, WidgetExt, ContainerExt};
use gdk::EventMask;


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


    /// removes a binding for an id
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
        if self.map.len() < id.0  {
            // check that the base index is within range
            Err(IDError::IDOutOfRange)
        } else if self.map.len() < replacement.0 {
            // check that the replacement index is within range
            Err(IDError::IDOutOfRange)
        } else {

            // grab the old id mapping
            let (ref mut timestamp, ref mut maybe_id) = self.map[id.0];

            // check that the timestamps match
            if *timestamp != id.1 {
                return Err(IDError::IDOutDated);
            }

            // and for the replacement id
            if self.map[replacement.0].0 != replacement.1 {
                return Err(IDError::IDOutDated);
            }

            // delete the old raw id as it has now been removed
            if let Some(old_raw_id) = maybe_id.take() {
                // simple sanity check, this function should only be called given one valid id to be removed, and one valid id to be removed
                assert!(self.map[replacement.0].1.is_some());

                // update the replacement index to point to raw_id
                self.map[replacement.0].1 = Some(old_raw_id);

                // increment the timestamp to indivalidate the remove id
                *timestamp += 1;

                // register this index as a potential empty space
                if self.next_empty_index.is_none() {
                    self.next_empty_index = Some(id.0);
                }
                // update the number of empty spaces
                self.empty_spaces += 1;

                Ok(())
            } else {
                // unlikely case, timestamps match, but old value has been removed
                Err(IDError::IDDeleted)
            }
        }
    }
}
