use crate::*;

struct BncMap{
    pub tilemap: HashMap<String,Vec<String>>,

}
impl BncMap{
    pub fn new() -> BncMap{
        BncMap{
            tilemap: HashMap::new(),


        }
    }

}
