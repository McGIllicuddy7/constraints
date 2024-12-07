pub type TileType = i32;
use crate::images::ByteImage;
use std::collections::HashMap;
#[derive(Clone)]
pub struct TileSet{
    pub tile_size:usize, 
    pub tiles:HashMap<TileType, ByteImage>,
}