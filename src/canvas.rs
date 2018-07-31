use graphic::Graphic;
use profile::Profile;

pub struct Weightsys {
    resolution: f64,
    grid: Vec<Vec<f64>>,
}

impl Weightsys {
    pub fn new(width: f64, height: f64, resolution: f64) -> Weightsys {
        Weightsys {
            resolution,
            grid: vec![vec![0.0; (height/resolution) as usize]; (width/resolution) as usize],
        }
    }

    pub fn calculate_and_collect_grid(
            &mut self,
            graphics: &Vec<Graphic>) 
            -> &Vec<Vec<f64>> 
    {        
        let mut col: usize;
        let mut row: usize;
        for graphic in graphics {
            col = graphic.focal_point().0 as usize % self.resolution as usize;
            row = graphic.focal_point().1 as usize % self.resolution as usize;
            self.grid[col][row] += graphic.weight();
        }

        &self.grid
    }
}

pub struct Canvas {
    width: f64,
    height: f64,
    aspect_ratio: f64,
    center_of_mass: (f64, f64),
    graphics: Vec<Graphic>,
    profile: Profile,
    weightsys: Weightsys,
}

impl Canvas {
    pub fn new(width: f64, height:f64) -> Canvas {
        Canvas {
            width,
            height,
            aspect_ratio: width/height,
            center_of_mass: (width/2.0, height/2.0),
            graphics: Vec::new(),
            profile: Profile::new(),
            weightsys: Weightsys::new(width, height, 10.0)
        }
    }

    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    pub fn graphics(&self) -> &Vec<Graphic> {
        &self.graphics
    }

    pub fn add_graphic(&mut self, graphic: Graphic) {
        self.graphics.push(graphic);
    }

    pub fn dimensions(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    pub fn get_weight_grid(&mut self) -> &Vec<Vec<f64>> {
        &self.weightsys.calculate_and_collect_grid(&self.graphics)
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn set_center_of_mass(&mut self, cx: f64, cy: f64) {
        self.center_of_mass = (cx, cy);
    }

    pub fn center_of_mass(&self) -> (f64, f64) {
        self.center_of_mass
    }
}