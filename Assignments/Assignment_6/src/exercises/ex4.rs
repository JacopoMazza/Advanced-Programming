#[derive(Clone)]
pub struct PublicStreetLight{
    pub id: i32,
    pub on: bool,
    pub burn_out: bool,
}

pub struct PublicIllumination{
    lights: Vec<Box<PublicStreetLight>>,
}


impl PublicStreetLight{
    pub fn new(id: i32, on: bool, burn_out: bool) -> PublicStreetLight{
        Self {
            id,
            on,
            burn_out,
        }
    }
}

impl Default for PublicStreetLight{
    fn default() -> PublicStreetLight{
        Self{
            id: 0,
            on: false,
            burn_out: false,
        }
    }
}

impl Default for PublicIllumination{
    fn default() -> PublicIllumination{
        Self {
            lights: Vec::new(),
        }
    }
}

impl Iterator for PublicIllumination{

    type Item = Box<PublicStreetLight>;

    fn next(&mut self) -> Option<Self::Item> {

        if let Some(pos) = self.lights.iter().position(|light| light.burn_out){
            Some(self.lights.remove(pos))
        }
        else{
            None
        }
    }
}