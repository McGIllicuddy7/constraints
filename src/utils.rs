


use rand::{thread_rng, RngCore};
use raylib::math::{Rectangle, Vector2};

#[allow(unused)]
pub const OFFSETS:[(isize,isize);8] = [(-1,-1), (0,-1), (1,-1), (-1,0), (1,0), (-1,1), (0,1),(1,1)];
#[allow(unused)]
pub const OFFSET_NAMES:[&str; 8] = ["top left", "top center", "top right", "middle left", "middle right", "bottom left", "bottom center", "bottom right"];
pub const MINUS_INDICES:[usize; 8] = [7, 6, 5, 4, 3, 2,1,0];

#[allow(unused)]
pub fn slice_rand_select<T:Clone>(values:&[(T, f64)])->T{
    let total = values.iter().map(|i| i.1).fold(0.0, |a:f64, b:f64| {a+b});
    let rnd= (thread_rng().next_u32()%10000)  as f64 /10000.0;
    let mut idx = 0;
    let mut base =0.0;
    while (values[idx].1+base)/total<rnd{
        base += values[idx].1;
        idx += 1;
    }
    values[idx].0.clone()
}


#[allow(unused)]
pub fn slice_intersection<T:Clone+PartialEq>(a:&[T], b:&[T], is_equal:&impl Fn (&T,&T)->bool)->Vec<T>{
    let mut out = Vec::new();
    let contains = |a:&[T], b:&T|{
        for i in a{
            if is_equal(i,b){
                return true;
            }
        }
        false
    };
    for i in a{
        if contains(b, i){
            out.push(i.clone());
        }
    }
    out
}
#[allow(unused)]
pub fn check_collision_line_rect(start:Vector2, end:Vector2, rect:&Rectangle)->bool{
    let col = raylib::check_collision_lines;
    let c1 = Vector2::new(rect.x,rect.y);
    let c2 = Vector2::new(rect.x+rect.width,rect.y);
    let c3 = Vector2::new(rect.x,rect.y+rect.height);
    let c4 = Vector2::new(rect.x+rect.width,rect.y+rect.height);
    rect.check_collision_point_rec(start)|| rect.check_collision_point_rec(end) || col(start, end, c1, c2).is_some() || col(start, end, c1, c3).is_some() || col(start,end, c2, c4).is_some() || col(start,end, c3, c4).is_some()
}

#[allow(unused)]
pub fn point_distance_to_line(point:Vector2, start:Vector2, end:Vector2)->f32{
    let x0 = point.x;
    let x1 = start.x;
    let x2 = end.x;
    let y0 = point.y;
    let y1 = start.y;
    let y2 = end.y;
    ((y2-y1)*x0-(x2-x1)*y0+x2*y1-y2*x1).abs()/(((y2-y1)*(y2-y1)+(x2-x1)*(x2-x1)).sqrt())
}