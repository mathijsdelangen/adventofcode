pub struct Point3 {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Point3 {
    pub fn is_neighbor(self, other: &Point3) -> bool {
        return (self.x as i32 - other.x as i32 ).abs() <= 1 
            && (self.y as i32 - other.y as i32 ).abs() <= 1 
            && (self.z as i32 - other.z as i32 ).abs() <= 1
            && *other != self;
    }
}

impl PartialEq for Point3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Copy for Point3 { }
impl Clone for Point3 {
    fn clone(&self) -> Point3 {
        Point3 {x: self.x, y: self.y, z: self.z}
    }
}

pub struct Cube {
    pub width: usize,
    pub height: usize,
    pub occupancies : Vec<Point3>,
}

pub fn is_present(occupancies: &Vec<Point3>, p: &Point3) -> bool {
    return occupancies.iter().any(|o| o.x == p.x && o.y == p.y && o.z == p.z);
}

pub fn count_occupied_neighbors(occupancies: &Vec<Point3>, p : Point3) -> usize {
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
                    '#' => points_in_cube.push(Point3 {x: (x+1), y: (y+1), z: 1}),
                    _  => (),
                }
            }
        }

        return Cube {width: width, height: 3, occupancies: points_in_cube}
    }

    pub fn print(&self) {
        for z in 0..self.height {
            println!("At height z={}", (z as i32 - self.height as i32 / 2));
            for y in 0..self.width {
                for x in 0..self.width {
                    if is_present(&self.occupancies,&Point3 {x:x, y:y, z:z}) { print!("#") }else { print!(".") }
                }
                println!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_is_neighbor(){
        assert_eq!(true, Point3 {x:1, y:2, z:3}.is_neighbor(&Point3 {x:2, y:2, z:2}));
        assert_eq!(true, Point3 {x:1, y:2, z:3}.is_neighbor(&Point3 {x:0, y:2, z:3}));
        assert_eq!(false, Point3 {x:2, y:2, z:2}.is_neighbor(&Point3 {x:2, y:2, z:2}));
    }
}