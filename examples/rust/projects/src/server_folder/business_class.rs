#[derive(Debug)]
pub struct BusinessObject {
    pub something: char,
    pub lastlast: char,
}
impl BusinessObject {
    pub fn new() -> BusinessObject {
        BusinessObject {
            something: 's',
            lastlast: 'l',
        }
    }
}
