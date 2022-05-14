#[derive(Debug)]
pub struct Cow {
    pub name: String,
    pub valor_compra : f32,
    pub valor_venda : f32,
    pub capacity : u8,
    pub milk : Vec<Milk>
}

impl Cow {
    pub fn spread (&self) -> f32 {
        self.valor_compra - self.valor_venda
    }
}

#[derive(Debug)]
pub struct Milk {
    pub price : f32,
    pub temp : [String; 2]
}