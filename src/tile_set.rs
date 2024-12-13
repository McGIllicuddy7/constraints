pub type TileType = i32;
use raylib::{color, math::Vector2};
use serde_derive::Serialize;

use crate::{images::ByteImage, utils};
use std::{collections::{HashMap, HashSet}, f32::consts::PI};
#[derive(Clone, Serialize)]
pub struct TileSet{
    pub tile_size:usize, 
    pub tiles:HashMap<TileType, ByteImage>,
}

#[allow(unused)]
#[derive(Clone)]
struct StreetTile{
    roads:Vec<Vec<Vector2>>,
    image:ByteImage,
}

#[allow(unused)]
fn make_street_tile(roads:Vec<Vec<Vector2>>, pixel_size:usize,count:usize)->StreetTile{
    let road_width:f32 = 5.0;
    let mut out_image = ByteImage::new_from_color(color::Color::BURLYWOOD, pixel_size,pixel_size);
    
    for i in &roads{
        for j in 0..i.len()-1{
            let start = i[j]*pixel_size as f32;
            let end = i[j+1]*pixel_size as f32;
            for y in 0..pixel_size{
                for x in 0..pixel_size{
                    let loc = Vector2::new(x as f32, y as f32);
                    if utils::point_distance_to_line(loc, start, end)<road_width{
                        out_image[y][x] = color::Color::DARKBROWN;
                    }
                }
            }
        }
    }
    for px in 0..2{
        for py in 0..2{
            let location = Vector2::new(pixel_size as f32/4.0 + px as f32* pixel_size as f32/2.0,  pixel_size as f32/4.0 + px as f32* pixel_size as f32/2.0);
            let rect = utils::rectangle_centered(location.x, location.y,16.0, 16.0);
            out_image.draw_rectangle(&rect,&color::Color::DARKBROWN);
        }
    }
    StreetTile{roads:roads, image:out_image}
}

#[allow(unused)]
fn calc_street_tile_bounds(tiles:&[StreetTile])-> HashMap<TileType,Vec<HashSet<TileType>>>{
    let mut out = HashMap::new();
    for i in 0..tiles.len() as TileType{
        out.insert(i, Vec::new());
    }
    for i in 0..tiles.len() as TileType{
        for j in 0..8{
            let mut tmp = HashSet::new();
            for k in 0..tiles.len() as TileType{
                let tmp_vec:Vec<Vec<Vector2>> = tiles[k as usize].roads.iter().map(|i| i.iter().map(|a| *a+Vector2::new(utils::OFFSETS[j].0 as f32, utils::OFFSETS[j].1 as f32 )).collect()).collect();
                let inter = utils::slice_intersection(&tmp_vec, &(tiles[i  as usize].roads), &|a:&Vec<Vector2>, b:&Vec<Vector2>|{ utils::slice_intersection(&a, &a, &|a:&Vector2, b:&Vector2| {a.distance_to(*b)<0.01}).len()>0});
                let t:&str = utils::OFFSET_NAMES[j];
                if t == "top center" || t == "middle left" ||t == "middle right" || t == "bottom center"{
                    if inter.len() >0{
                        tmp.insert(k);
                    }
                } else{
                    if inter.len() >0{
                        tmp.insert(k);
                    }
                }
                if tiles[k as usize].roads.len() == 0 || tiles[i as usize].roads.len() == 0{
                    tmp.insert(k);
                }
            }
            assert!(tmp.len()>0);
            out.get_mut(&i).expect("must exist").push(tmp);
        }
    }
    out
}
#[allow(unused)]
pub fn make_city_tile_set()->(TileSet, HashMap<TileType,Vec<HashSet<TileType>>>){   
    let mut idx = 0;
    let mut tiles = Vec::new();
    let tile_size = 50;
    let tcount = 1;
    let rotator = |mut locations:Vec<Vector2>, amount_to_rotate:i32|{
        for i in &mut locations{
            let mut delta = *i - Vector2::new(0.5, 0.5);
            delta.rotate(PI *0.5 * amount_to_rotate as f32);
            *i = delta+Vector2::new(0.5,0.5);
        }
        locations
    } ;
    let empty = StreetTile{roads:vec![], image:ByteImage::new_from_color(color::Color::BURLYWOOD, tile_size,tile_size)};
    tiles.push(empty);
    for i in 0..4{
        let base = Vector2::new(0.5, 0.5);
        let t0 = make_street_tile(vec![rotator(vec![Vector2::new(1.0, 0.5), Vector2::new(0.0, 0.5)],i)],100, tcount);
        tiles.push(t0);
    }
    let base = Vector2::new(0.5, 0.5);
    let mut tile_map = HashMap::new();
    for i in 0..tiles.len(){
        tiles[i].image.export(&format!("images/{i}.png"));
        tile_map.insert(i as i32, tiles[i].image.clone());
    }
    let constraints = calc_street_tile_bounds(&tiles);
    let tile_set = TileSet{tile_size, tiles:tile_map};
    (tile_set, constraints)
}