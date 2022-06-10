#[derive(Debug)]
pub struct Cow {
    pub n: String,
    pub vc : f32,
    pub vv : f32,
    pub cp : u8,
    pub m : Vec<Milk>
}

impl Cow {
    pub fn new (n: String, vc: f32, vv: f32, cp: u8, m: Vec<Milk>) -> Self {
        Self { n, vc, vv, cp, m}
    }

    pub fn spread (&self) -> f32 {self.vc - self.vv}
}

#[derive(Debug)]
pub struct Milk {
    pub price : f32,
    pub temp : [String; 2]
}