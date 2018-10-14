macro_rules! Coord {
    ($x:expr, $y:expr, $z:expr, $weight:expr) => {{
        WeightedCoordinate{
            weight: $weight,
            coordinate: Coordinate {x: $x, y: $y, z: $z}
        }
    }};
    ($x:expr, $y:expr, $z:expr) => {{
        Coordinate{x: $x, y: $y, z: $z}
    }};
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Coordinate {
    pub x: f64, 
    pub y: f64, 
    pub z: f64
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct WeightedCoordinate {
    pub weight: f64, 
    pub coordinate: Coordinate
}

pub fn calc_center_of_mass(points: Vec<WeightedCoordinate>) -> WeightedCoordinate {
    let mut center_point: Coordinate = Coord!(0.0, 0.0, 0.0);
    let mut total_mass = 0.0;

    for point in points {
        center_point.x += point.weight*(point.coordinate).x;
        center_point.y += point.weight*(point.coordinate).y;
        center_point.z += point.weight*(point.coordinate).z;

        total_mass += point.weight;
    }

    if total_mass > 0.0 {
        center_point.x /= total_mass;
        center_point.y /= total_mass;
        center_point.z /= total_mass;
    }

    WeightedCoordinate{
        weight: total_mass, 
        coordinate: center_point
    }
}