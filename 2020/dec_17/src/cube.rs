pub struct Point4 {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub w: usize
}

impl Point4 {
    pub fn is_neighbor(self, other: &Point4) -> bool {
        return (self.x as i32 - other.x as i32 ).abs() <= 1 
            && (self.y as i32 - other.y as i32 ).abs() <= 1 
            && (self.z as i32 - other.z as i32 ).abs() <= 1
            && (self.w as i32 - other.w as i32 ).abs() <= 1
            && *other != self;
    }
}

impl PartialEq for Point4 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl Copy for Point4 { }
impl Clone for Point4 {
    fn clone(&self) -> Point4 {
        Point4 {x: self.x, y: self.y, z: self.z, w: self.w}
    }
}

pub struct Cube {
    pub width: usize,
    pub height: usize,
    pub max_w: usize,
    pub occupancies : Vec<Point4>,
}

pub fn is_present(occupancies: &Vec<Point4>, p: &Point4) -> bool {
    return occupancies.iter().any(|o| o.x == p.x && o.y == p.y && o.z == p.z && o.w == p.w);
}

pub fn count_occupied_neighbors(occupancies: &Vec<Point4>, p : Point4) -> usize {
    return occupancies.iter().filter(|&o| p.is_neighbor(o)).count();
}

impl Cube {
    pub fn new_from_str(description: &str) -> Cube {
        let lines : Vec<&str> = description.split("\r\n").collect();
        let width = lines[0].chars().count() + 2;

        let mut points_in_cube = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            for (x,c) in line.chars().enumerate() {
                match c {
                    '#' => points_in_cube.push(Point4 {x: (x+1), y: (y+1), z: 1, w: 1}),
                    _  => (),
                }
            }
        }

        return Cube {width: width, height: 3, max_w: 3, occupancies: points_in_cube}
    }

    pub fn print(&self) {
        for w in 0..self.max_w {
            for z in 0..self.height {
                println!("At height z={}, w={}", (z as i32 - self.height as i32 / 2), (w as i32 - self.max_w as i32 / 2));
                for y in 0..self.width {
                    for x in 0..self.width {
                        if is_present(&self.occupancies,&Point4 {x:x, y:y, z:z, w:w}) { print!("#") }else { print!(".") }
                    }
                    println!();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_is_neighbor(){
        assert_eq!(true, Point4 {x:1, y:2, z:3, w:0}.is_neighbor(&Point4 {x:2, y:2, z:2, w:0}));
        assert_eq!(true, Point4 {x:1, y:2, z:3, w:0}.is_neighbor(&Point4 {x:0, y:2, z:3, w:0}));
        assert_eq!(false, Point4 {x:2, y:2, z:2, w:0}.is_neighbor(&Point4 {x:2, y:2, z:2, w:0}));
    }
}