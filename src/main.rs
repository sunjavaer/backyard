use crate::garden::vegetables::Size;
use garden::vegetables::{Asparagus, AsparagusV2};
pub mod garden;

fn main() {
    let asparagus = Asparagus {};
    let asparagus_v2 = AsparagusV2 {};
    let size = Size::Small;
    
    println!("Hello, world!. {:?}", size);
}
