use std::str::FromStr;
use crate::day10::Point;
use std::fmt::Error;
//position=< 9,  1> velocity=< 0,  2>
impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut iter = s.split("> velocity=<");
        let position_str  = iter.next().unwrap();
        let vel_str  = iter.next().unwrap();
        
        let position_str = &position_str[10..];
        let mut pi = position_str.split(", ");
        let x :i32  = pi.next().map(|px| px.parse().unwrap_or(0)).unwrap();
        let y :i32  = pi.next().map(|py| py.parse().unwrap_or(0)).unwrap();

        let vel_str = &position_str[..vel_str.len()-1];
        let mut vi = vel_str.split(", ");
        let vx :i32  = vi.next().map(|px| px.parse().unwrap_or(0)).unwrap();
        let vy :i32  = vi.next().map(|py| py.parse().unwrap_or(0)).unwrap();
        
        
        Ok(Point {
            position: (x,y),
            velocity: (vx,vy)
        })
    }
}

pub fn board_size(points : &Vec<Point>) -> (i32, i32, i32, i32) {
    let minx =points.iter()
        .map(|p| p.position.0)
        .min()
        .unwrap();

    let maxx =points.iter()
        .map(|p| p.position.0)
        .max()
        .unwrap();

    let miny =points.iter()
        .map(|p| p.position.1)
        .min()
        .unwrap();

    let maxy =points.iter()
        .map(|p| p.position.1)
        .max()
        .unwrap();
    let width = minx.abs() + maxx.abs(); 
    let height = miny.abs() + maxy.abs(); 
    (
        width, height, width - maxx.abs(), height - maxy.abs()

    )
}



