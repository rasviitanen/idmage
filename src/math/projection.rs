use rulinalg::matrix::Matrix;
use rulinalg::matrix::BaseMatrix;

use std::f64;

pub struct Camera {
    pub position: (f64, f64, f64),
    pub orientation: (f64, f64, f64),
    pub display_surface: (f64, f64, f64),
}

impl Camera {
    pub fn new(position: (f64, f64, f64), 
        orientation: (f64, f64, f64), 
        display_surface: (f64, f64, f64)) -> Camera 
    {
        Camera {
            position,
            orientation,
            display_surface,
        }
    }
}

pub fn project_3d_point_on_2d_surface(camera: &Camera, point: (f64, f64, f64)) -> (f64, f64) {
    let tait_bryan_x = Matrix::new(3, 3, vec![
        1.0,    0.0,                            0.0,
        0.0,    camera.orientation.0.cos(),    camera.orientation.0.sin(),
        0.0,    -camera.orientation.0.sin(),   camera.orientation.0.cos()
    ]);

    let tait_bryan_y = Matrix::new(3, 3, vec![
        camera.orientation.1.cos(),     0.0,    -camera.orientation.1.sin(),
        0.0,                            1.0,    0.0,
        camera.orientation.1.sin(),     0.0,    camera.orientation.1.cos()
    ]);

    let tait_bryan_z = Matrix::new(3, 3, vec![
        camera.orientation.2.cos(),    camera.orientation.2.sin(),    0.0,
        -camera.orientation.2.sin(),   camera.orientation.2.cos(),    0.0,
        0.0,                           0.0,                           1.0
    ]);

    let to_camera_vector = Matrix::new(3,1, vec![
        point.0 - camera.position.0, 
        point.1 - camera.position.1,
        point.2 - camera.position.2,
    ]);

    let camera_coord = &tait_bryan_x * &tait_bryan_y * &tait_bryan_z * &to_camera_vector;

unsafe {
    let z_ratio = camera.display_surface.2/camera_coord.get_unchecked([0, 2]);

    println!("{:?}", camera_coord);
    println!("Z:RATIO  {:?}", z_ratio);
    println!("camera display surface:::  {:?}", camera.display_surface);

    println!("zratio * cameracoord:::  {:?}", z_ratio*camera_coord.get_unchecked([0, 0]));

    // new x,y on the plane
    (z_ratio*camera_coord.get_unchecked([0, 0]) + camera.display_surface.0,
        z_ratio*camera_coord.get_unchecked([0, 1]) + camera.display_surface.1)

}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project() {
        let camera = Camera::new((1.0, 1.0, 1.0), (1.0, 1.0, 1.0));
        let projected_point = 
            project_3d_point_on_2d_surface(&camera, (10.0, 10.0, 100.0));
        println!("{:?}", projected_point);
        assert_eq!(0, 1);
    }
}