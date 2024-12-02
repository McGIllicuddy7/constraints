use rand::{thread_rng, RngCore};

#[allow(unused)]
pub const OFFSETS:[(isize,isize);8] = [(-1,-1), (0,-1), (1,-1), (-1,0), (1,0), (-1,1), (0,1),(1,1)];
#[allow(unused)]
pub const OFFSET_NAMES:[&'static str; 8] = ["top left", "top center", "top right", "middle left", "middle right", "bottom left", "bottom center", "bottom right"];
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
    return values[idx].0.clone();
}