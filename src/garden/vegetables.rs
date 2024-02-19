#[derive(Debug)]
pub struct Asparagus {}

#[derive(Debug)] 
pub struct AsparagusV2 {}

#[derive(Debug)] 
pub enum Size {
    Small,
    Medium,
    Large
}

#[derive(Debug)] 
struct PrivateStruct {
    id: i32,
    name: String
}

// pub fn new_private_struct(id: i32, name: String) -> PrivateStruct {
//     PrivateStruct {
//         id,
//         name
//     }
// }