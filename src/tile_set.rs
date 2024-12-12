pub type TileType = i32;
use raylib::{color, math::{Rectangle, Vector2}};
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
    roads:Vec<Vector2>,
    image:ByteImage,
}

#[allow(unused)]
fn make_street_tile(roads:Vec<Vector2>, pixel_size:usize,count:usize)->StreetTile{
    let countf =count as f32;
    let building_size = pixel_size as f32/countf;
    let ps = pixel_size as f32;
    let dv = 1.00;
    let road_width =ps*0.1;
    let base_buildings:Vec<Rectangle> = {
        let mut tmp = Vec::new();
        for x in 0..count{
            for y in 0..count{
                let i = Rectangle { x:x as f32*building_size, y: y as f32*building_size, width: building_size, height: building_size};
                let s = i.width/8.0;
                tmp.push(Rectangle::new(i.x+s/dv/2.0,i.y+s/dv/2.0, i.width-s*2.0/dv, i.height-s*2.0/dv));
            }

        }
        tmp
    };
    let buildings = {
        let mut tmp = Vec::new();
        for i in base_buildings{
            let mut col = false;
            for j in 0..roads.len()-1{
                let jp = j+1;
                if utils::check_collision_line_rect(roads[j]*ps,roads[jp]*ps, &i){
                    col = true;
                    break;
                }
            }
            if !col{
                tmp.push(i);
            }

        } 
        tmp
    };
    let mut img = 
    raylib::prelude::Image::gen_image_color(pixel_size as i32, pixel_size as i32, color::Color::BURLYWOOD);
    for b in &buildings{
        //img.draw_rectangle(b.x as i32+1, b.y as i32+1, b.width as i32, b.height as i32, color::Color::DIMGRAY);
    }
    let mut drew = false;
    for j in 0..roads.len()-1{
        let jp = j+1;
        let p1 = roads[j]*ps;
        let p2 = roads[jp]*ps;
        for y in 0..img.height{
            for x in 0..img.width{
                let dist = utils::point_distance_to_line(Vector2::new(x as f32, y as f32),p1, p2);
                if dist<=road_width{
                    img.draw_pixel(x, y, color::Color::DARKBROWN);
                    drew = true;
                }

            }
        }
        assert!(drew);
    }
    StreetTile { roads, image: ByteImage::new(&mut img) }
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
                let tmp_vec:Vec<Vector2> = tiles[k as usize].roads.iter().map(|i| *i +Vector2::new(utils::OFFSETS[j].0 as f32, utils::OFFSETS[j].1 as f32 ) ).collect();
                let inter = utils::slice_intersection(&tmp_vec, &(tiles[i  as usize].roads), &|a:&Vector2, b:&Vector2|{(*a-*b).length_sqr()<0.001|| (*a-*b).length_sqr()>=2.0});
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
        let t0 = make_street_tile(rotator(vec![Vector2::new(1.0, 0.0), Vector2::new(0.0, 0.0)],i),100, tcount);
        tiles.push(t0);
        let t1 = make_street_tile(rotator(vec![Vector2::new(1.0, 0.0), Vector2::new(0.0, 0.0)],i),100, tcount*2);
        tiles.push(t1);
    }
    let base = Vector2::new(0.5, 0.5);
    for i in 0..4{
        let base = Vector2::new(0.5, 0.5);
        let t0 = make_street_tile(rotator(vec![Vector2::new(1.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 1.0)],i),100, tcount);
        tiles.push(t0);
        let t1 = make_street_tile(rotator(vec![Vector2::new(1.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 1.0)],i),100, tcount*2);
        tiles.push(t1);
    }
    for i in 0..4{
        let base = Vector2::new(0.5, 0.5);
        let t0 = make_street_tile(rotator(vec![Vector2::new(1.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 1.0), Vector2::new(1.0, 1.0)],i),100, tcount);
        tiles.push(t0);
        let t1 = make_street_tile(rotator(vec![Vector2::new(1.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 1.0), Vector2::new(1.0, 1.0)],i),100, tcount*2);
        tiles.push(t1);
    }
    let mut tile_map = HashMap::new();
    for i in 0..tiles.len(){
        tiles[i].image.export(&format!("images/{i}.png"));
        tile_map.insert(i as i32, tiles[i].image.clone());
    }
    let constraints = calc_street_tile_bounds(&tiles);
    let tile_set = TileSet{tile_size, tiles:tile_map};
    (tile_set, constraints)
}