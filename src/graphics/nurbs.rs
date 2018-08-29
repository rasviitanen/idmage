use graphic::Graphic;
use math::projection;
use math::projection::Camera;

macro_rules! bezier {
    ($command:expr, $pos:expr) => {{
        Anchor {
            command: $command,
            position: $pos,
            control: None,
        }
    }};
    ($command:expr, $pos:expr, $ctrl:expr) => {{
        Anchor {
            command: $command,
            position: $pos,
            control: Some($ctrl)
        }
    }};
}

pub struct Anchor {
    pub command: char,
    pub position: (f64, f64, f64),
    pub control: Option<(f64, f64, f64)>
}

pub fn nurbs_surface(camera: &Camera, anchors: &Vec<Anchor>) -> Graphic {
    let mut nurbs_surface = Graphic::new("path");
    let mut path = "".to_string();

    let mut last_command = ' ';

    let mut projected_position: (f64, f64);

    for anchor in anchors {
        if last_command != anchor.command {
            path += &anchor.command.to_string();
            last_command = anchor.command;
        } else {
            path += ",";
        }
        projected_position = projection::project_3d_point_on_2d_surface(&camera, anchor.position);
        path += &projected_position.0.to_string();
        path += " ";
        path += &projected_position.1.to_string();

        match anchor.control {
            Some(control) => {
                projected_position = projection::project_3d_point_on_2d_surface(&camera, control);
                path += ",";
                path += &projected_position.0.to_string();
                path += " ";
                path += &projected_position.1.to_string();
            }
            _ => {}
        }

    };
    
    nurbs_surface.add_attr(ATTR!("d", path));
    nurbs_surface.add_attr(ATTR!("stroke", "yellow"));
    nurbs_surface.add_attr(ATTR!("fill", RGBA!(0, 0, 0, 0.5)));
    nurbs_surface
}
