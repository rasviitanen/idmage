use agent::agent::Agent;
use canvas::Canvas;

fn divide_rule_of_thirds(canvas: &mut Canvas) {
    let (width, height) = canvas.dimensions();
    canvas.add_tile((0.0, 0.0), (width/3.0, height));
    canvas.add_tile((width/3.0, height), (2.0*width/3.0, height));
    canvas.add_tile((2.0*width/3.0, height), (width, height));
}

pub struct Tiler {
    tiling_constant: usize,
}

impl Tiler {
    pub fn new() -> Tiler {
        Tiler {
            tiling_constant: 10,
        }
    }
}

impl Agent for Tiler {
    fn update(&self, canvas: &mut Canvas) {
        if canvas.tiles().len() == 0 {
            if canvas.aspect_ratio() >= 21.0/9.0 {
                // ULTRAWIDE
                divide_rule_of_thirds(canvas);
            } else if canvas.aspect_ratio() >= 16.0/9.0 {
                // WIDESCREEN
                divide_rule_of_thirds(canvas);
                println!("{:?}", "Widescreen");
            } else if canvas.aspect_ratio() >= 4.0/3.0 {
                // STANDARD
                divide_rule_of_thirds(canvas);
                println!("{:?}", "Standard");
            }
        }
    }
}