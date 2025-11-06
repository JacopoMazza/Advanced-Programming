
#[derive(Debug)]
struct Point{
   pub x:f32,
    pub y:f32
}

impl Point{
    pub fn new(x: f32,y:f32) -> Point {
        Point {
            x,
            y
        }
    }

    pub fn distance(&self, p2:&Point) -> f32 {
     ((self.x -p2.x).powf(2f32) +(self.y -p2.y).powf(2f32) ).sqrt()
    }
}


