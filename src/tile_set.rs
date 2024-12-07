pub type TileType = i32;
use serde_derive::Serialize;

use crate::images::ByteImage;
use std::collections::HashMap;
#[derive(Clone, Serialize)]
pub struct TileSet{
    pub tile_size:usize, 
    pub tiles:HashMap<TileType, ByteImage>,
}
