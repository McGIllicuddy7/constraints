use std::{collections::{HashMap, HashSet}, sync::Arc};

use rand::{thread_rng, RngCore};
use crate::utils;
use crate::utils::OFFSETS;
use crate::utils::MINUS_INDICES;
type TileType = i32;

#[derive(Clone, Debug)]
pub struct Grid{
    values:Box<[TileType]>, 
    height:usize, 
    width:usize,
}

impl Grid{
    pub fn new(height:usize, width:usize)->Self{
        let mut tmp: Vec<TileType> = Vec::new();
        tmp.reserve_exact(height*width);
        for _ in 0..width{
            for _ in 0..height{
                tmp.push(-1);
            }
        }
        let values:Box<[TileType]> = tmp.into();
        Self{values, height, width}
    }

    pub fn get_sq(&self,x:usize, y:usize)->&TileType{
        assert!(x<self.width&& y<self.height);
        &self.values[y*self.width+x]
    }

    pub fn get_sqmut(&mut self, x:usize, y:usize)->&mut TileType{
        assert!(x<self.width&& y<self.height);
        &mut self.values[y*self.width+x]
    }
    pub fn to_str(&self)->String{
        let mut out = String::new();
        for y in 0..self.height{
            for x in 0..self.width{
                out += &format!("{}", *self.get_sq(x, y));
                if x != self.width-1{
                    out += ",";
                }
            }
            out += "\n";
        }
        out
    }
}

#[derive(Clone)]
pub struct GridConstraint{
    //returns true if the state is valid, false if it's invalid
    pub constraints_sat:Arc<dyn Fn(&Grid,TileType, usize, usize)->bool>,
    pub debug_fn:Arc<dyn Fn()->String>,
}
impl std::fmt::Debug for GridConstraint{
    fn fmt(&self, formatter:&mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { let _ = formatter.write_str(&(self.debug_fn)()); Ok(())}
}
impl GridConstraint{
    pub fn new(func:Arc<dyn Fn(&Grid, TileType,usize, usize)->bool>,debug_fn:Arc<dyn Fn()->String>)->Self{
        Self{constraints_sat:func, debug_fn}
    }

    // tile types; directions within tile tiles; allowed types per direction
    pub fn new_from_borders(constraints:HashMap<TileType,Vec<HashSet<TileType>>>)->Self{
        let debug_constraints = constraints.clone();
        let func = move |grid:&Grid, tile_type:TileType, x:usize, y:usize|{
            for i in 0..8{
                let (dx,dy) = OFFSETS[i];
                if x as isize+dx<0 || y as isize+dy<0{
                    continue;
                }
                if (x as isize +dx) as usize>= grid.width || (y as isize+dy) as usize>= grid.height{
                    continue;
                }
                let tt = *grid.get_sq((x as isize+dx) as usize, (y as isize+dy) as usize);
                if tt< 0{
                    continue;
                }
                if tt as usize>=constraints.len(){
                    panic!("error found out of bounds tile_type");
                } 
                let tc = constraints.get(&tt).unwrap();
                let j = MINUS_INDICES[i];
                if !tc[j].contains(&tile_type){
                    //println!("error: constraint not satisfied, {} not allowed {}  of {}", tile_type, OFFSET_NAMES[j], tt);
                    return false
                }
            }
            true
        };
        let debug_fn = move||{
            format!("{:#?}", debug_constraints)
        };
        Self{constraints_sat:Arc::new(func), debug_fn:Arc::new(debug_fn)}
    }

    //returns true if the state is valid, false if it's invalid
    pub fn check_constraint(&self, grid:&Grid,test_value:TileType, x:usize, y:usize)->bool{
        if test_value == -1{
            return true;
        }
        (self.constraints_sat)(grid,test_value, x,y)
    }
    pub fn get_allowed_states(&self,grid:&Grid,allowed_states:&[TileType],x:usize, y:usize)->Vec<TileType>{
        let mut out = vec![];
        for i in allowed_states{
            if self.check_constraint(grid, *i, x, y){
                out.push(*i);
            }
        }
        out
    }
}

pub enum SelectionStrategy{PurelyRandom, FromDistribution{distribution:Box<[(TileType, f64)]>}, MinimizeEntropy, MaximizeEntropy}

#[derive(Clone, Debug)]
pub struct ConstraintSolver{
    grid:Grid, 
    constraints:Vec<GridConstraint>,
    allowed_states:Arc<[TileType]>, 
    allowed_neighbors_cache:HashMap<(usize,usize), Vec<TileType>>
}

impl ConstraintSolver{

    pub fn new(height:usize,width:usize, allowed_states:Arc<[TileType]>)->Self{
        Self{grid:Grid::new(height, width), constraints:Vec::new(), allowed_states, allowed_neighbors_cache:HashMap::new()}
    }

    pub fn new_with_constraints(height:usize, width:usize,allowed_states:Arc<[TileType]>, constraints:Vec<GridConstraint>)->Self{
        Self{grid:Grid::new(height, width), constraints, allowed_states,allowed_neighbors_cache:HashMap::new()} 
    }

    pub fn is_state_valid(&self)->bool{
        for y in 0..self.grid.height{
            for x in 0..self.grid.width{
                if !self.constraints[0].check_constraint(&self.grid,*self.grid.get_sq(x,y), x,y){
                    return false
                }
            }
        }
        true
    }

    pub fn new_from_data(data:&[TileType], height:usize, width:usize)->Self{
        let mut used:HashSet<TileType> = HashSet::new();
        let mut allowed_states:Vec<TileType> = Vec::new();
        for i in data{
            if !used.contains(i){
                used.insert(*i);
                allowed_states.push(*i);
            }
        }
        let mut constraints:Vec<GridConstraint> = Vec::new();
        let mut allowed_border = HashMap::new();
        for i in &allowed_states{
            let mut tmp_vec = Vec::new();
            for j in 0..8{
                let mut states = HashSet::new();
                let (dx, dy) = OFFSETS[j];
                for y in 0..height{
                    for x in 0..width{
                        if data[y*height+x] != *i{
                            continue;
                        }
                        if x as isize+dx<0 || y as isize+dy<0{
                            continue;
                        }
                        if (x as isize+dx) as usize>=width || (y as isize+dy) as usize>=height{
                            continue;
                        }
                        states.insert(data[((y as isize+dy) as usize)*height+(x as isize+dx) as usize]);
                    }
                }
                tmp_vec.push(states);
            }
            allowed_border.insert(*i,tmp_vec);
        }
        constraints.push(GridConstraint::new_from_borders(allowed_border));
        Self{grid:Grid::new(height, width), constraints, allowed_states:allowed_states.into(),allowed_neighbors_cache:HashMap::new()}
    }

    //sets the value at the location to the requested one, clears the cache of the value and all it's neighbors
    pub unsafe fn collapse_unchecked(&mut self, x:usize, y:usize, value:TileType){
        *(self.grid.get_sqmut(x, y)) = value;
        //self.allowed_neighbors_cache.clear();
    }

    //returns true on error
    pub unsafe fn collapse_unchecked_recursive(&mut self, x:usize, y:usize, value:TileType)->bool{
        *(self.grid.get_sqmut(x, y)) = value;
        let ix = x as isize;
        let iy = y as isize;
        self.allowed_neighbors_cache.remove(&(x,y));
        for i in 0..8{
            let dx = OFFSETS[i].0;
            let dy = OFFSETS[i].1;
            let sx = ix+dx;
            let sy = iy+dy;
            if sx< 0 || sy<0 || sx>= self.grid.width as isize || sy >= self.grid.height as isize{
                continue;
            }
            let vx = sx as usize;
            let vy = sy as usize;
            self.allowed_neighbors_cache.remove(&(vx,vy));
            if *(self.grid.get_sq(vx, vy)) != -1{
                continue;
            }
            let allowed = self.allowed_states_at(vx, vy);
            if allowed.len() == 1{
                let result = self.collapse_unchecked_recursive(vx, vy, allowed[0]);
                if result{
                    return true;
                }
            } else if allowed.is_empty(){
                return true;
            }
        } 
        false
    }
    pub fn check_collapse_allowed(&self, x:usize, y:usize, test_value:TileType)->bool{
        for i in &self.constraints{
            if !i.check_constraint(&self.grid, test_value, x, y){
                return false;
            }
        }
        true
    }

    pub fn allowed_states_at(&mut self,x:usize, y:usize)->Vec<TileType>{
        if let Some(v) = self.allowed_neighbors_cache.get(&(x,y)){
            return v.clone();
        }
        let mut out = vec![];
        for i in &self.constraints{
            let mut tmp = i.get_allowed_states(&self.grid, &self.allowed_states, x, y);
            out.append(&mut tmp);
        }
        self.allowed_neighbors_cache.insert((x,y), out.clone());
        out
    }

    //returns true if it reached an unreachable state
    pub fn collapse_all_determined(&mut self)->bool{
        let previous = self.grid.clone();
        let mut reset = true;
        while reset{
            reset = false;
            for x in 0..self.grid.width{
                for y in 0..self.grid.height{
                    if *(self.grid.get_sq(x, y)) != -1{
                        continue;
                    }
                    let al = self.allowed_states_at(x, y);
                    if al.is_empty(){
                        self.grid = previous;
                        return true;
                    }
                    if al.len() == 1{
                        unsafe{self.collapse_unchecked_recursive(x, y, al[0]);}
                        reset = true;
                    }
                }
            }
        }
        false
    }

    //returns Ok(true) if the collapse was allowed Ok(false) if collapsing reached an unreachable state returns an error
    pub fn attempt_collapse_to_value(&mut self,x:usize, y:usize, value:TileType)->Result<bool,()>{
        if self.check_collapse_allowed(x, y, value){
         
            let result =    unsafe{
                self.collapse_unchecked_recursive(x,y,value)
            };
            if result {
                Err(())
            } else if self.is_state_valid(){
                Ok(true)
            }
            else{
                Err(())
            }

        } else{
            Ok(false)
        }
    }

    //returns Ok(true) if the collapse was allowed Ok(false) reached an unreachable state returns an error
    pub fn collapse_location(&mut self, x:usize, y:usize, selction_mode:&SelectionStrategy)->Result<bool, ()>{
        let mut rand= thread_rng();
        let allowed_states = self.allowed_states_at(x, y).into_boxed_slice();
        if allowed_states.len() <1{
            return Err(());
        }
        let state = match selction_mode{
            SelectionStrategy::PurelyRandom=>{
                let i = rand.next_u64() as usize % allowed_states.len();
                allowed_states[i]
            }
            SelectionStrategy::FromDistribution {distribution }=>{
                 utils::slice_rand_select(distribution.as_ref())
            }
            SelectionStrategy::MaximizeEntropy=>{
                todo!()
            }
            SelectionStrategy::MinimizeEntropy=>{
                todo!()
            }
        };
        let result = unsafe {
            self.collapse_unchecked_recursive(x, y, state)
        };
        if result{
            return Err(());
        }
        Ok(true)
    }

    pub fn contains_undefined(&self)->bool{
        for i in self.grid.values.as_ref(){
            if *i == -1{
                return true;
            }
        }
        false
    }

    pub fn undefined_count(&self)->usize{
        let mut count = 0;
        for i in self.grid.values.as_ref(){
            if *i == -1{
                count += 1;
            }
        }
        count
    }

    pub fn collapse_lowest_entropy(&mut self, selection_mode:&SelectionStrategy)->Result<bool, ()>{
        let mut idxs:Vec<((usize,usize), usize)> = Vec::new();
        for y in 0..self.grid.height{
            for x in 0..self.grid.width{
                if *(self.grid.get_sq(x,y)) == -1{
                    idxs.push(((x,y), self.allowed_states_at(x, y).len()));
                }
            }
        }
        idxs.sort_unstable_by(|a,b|{
            if a<b{std::cmp::Ordering::Greater} else{ std::cmp::Ordering::Less}
            }
          );
        let x = idxs[0].0.0;
        let y = idxs[0].0.1;
        self.collapse_location(x, y, selection_mode)
    }

    pub fn collapse_fully(&mut self, selection_mode:&SelectionStrategy)->bool{
        let mut ud_count = self.undefined_count();
        while ud_count>0{
            //println!("ud_count:{ud_count}");
            let r = self.collapse_lowest_entropy(selection_mode);
            if r.is_err(){
                return false;
            }
            if r.is_ok() && !r.unwrap() {
                return false;
            }
            ud_count = self.undefined_count();
        }
        /*if !self.is_state_valid(){
            return false;
        }*/
        true
    }
}

#[test]
fn test_offsets(){
    for i in 0..8{
        let (x0,y0) = OFFSETS[i];
        let (x1, y1) = OFFSETS[MINUS_INDICES[i]];
        assert!(x1 == -x0);
        assert!(y1 == -y0);
    }
}

#[test]
fn test_initial_state_is_valid(){
    let height =10;
    let width = 10;
    let mut data =Vec::new();
    for i in 0..height{
        for j in 0..width{
            data.push(if (i+j)%2 == 0{0 as TileType} else{1 as TileType});
        }
    }
    let mut solve=
     ConstraintSolver::new_from_data(&data, height, width);
     for i in 0..height*width{     
        solve.grid.values[i] = data[i];
     }
     eprintln!("{:#?}", solve.constraints);
     eprintln!("{}", solve.grid.to_str());
     assert!(solve.is_state_valid());
}

#[allow(unused)]
pub fn test_collapse(){
    let height:usize =60;
    let width:usize = 60;
    let mut data =Vec::new();
    for i in 0..height{
        for j in 0..width{
            data.push( ((thread_rng().next_u32())%3) as TileType);
        }
    }
    let mut solve=
     ConstraintSolver::new_from_data(&data, height, width);
    assert!(solve.collapse_fully(&SelectionStrategy::PurelyRandom));
    eprintln!("{}", solve.grid.to_str());

}