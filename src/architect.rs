use super::layout;
use super::profiler;
use super::request;
use super::instructions::LayoutInstruction;
use graphic::Graphic;
use graphic::GraphicType;
use designer;

pub struct Tile<'a> {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    weight: i32,
    graphic_content: Vec<Graphic<'a>>,
}

impl<'a> Tile<'a> {
    pub fn new(x: f64, y: f64, width: f64, height: f64, weight: i32) -> Tile<'a> {
        Tile {
            x,
            y,
            width,
            height,
            weight,
            graphic_content: Vec::new(),
        }
    }

    pub fn get_weight(&self) -> &i32 {&self.weight}
    pub fn get_graphics(&mut self) -> &mut Vec<Graphic<'a>> {
        &mut self.graphic_content
    }
}

struct TileGrid<'a> {
    tiles: Vec<Tile<'a>>,
}

impl<'a> TileGrid<'a> {
    fn new() -> TileGrid<'a> {
        TileGrid{tiles: Vec::new()}
    }
    fn push(&mut self, tile: Tile<'a>) {
        self.tiles.push(tile);
    }
}

pub struct Blueprint<'a> {
    width: f64,
    height: f64,
    scaled_width: f64,
    scaled_height: f64,
    graphic_content: Vec<Graphic<'a>>,
    tile_grid: TileGrid<'a>,
    profile: profiler::Profile,
}

impl<'a> Blueprint<'a> {
    pub fn new(profile: profiler::Profile, request: request::Request<'a>) -> Blueprint<'a> {
        let mut bp = Blueprint {
            width: request.width,
            height: request.height,
            scaled_width: 1000.0 * request.width / request.width.max(request.height),
            scaled_height: 1000.0 * request.height / request.width.max(request.height),
            graphic_content: Vec::new(),
            tile_grid: TileGrid::new(),
            profile,
        };
        bp.populate(request);
        bp.split_canvas_to_tiles();
        let my_designer: designer::Designer;
        my_designer = designer::Designer::new();
        my_designer.start(&mut bp);
        bp
    }

    pub fn populate(&mut self, request: request::Request<'a>) {
        for graphic in request.content {
            self.graphic_content.push(graphic);
        }
    }

    pub fn split_canvas_to_tiles(&mut self) {
        // !TODO make smart
        let mut tile1 = Tile::new(0.0, 0.0, self.width, self.height, 1);
        self.tile_grid.push(tile1);
    }

    pub fn get_width(&self) -> f64 {
        self.width
    }
    pub fn get_height(&self) -> f64 {
        self.height
    }
    pub fn get_scaled_width(&self) -> f64 {
        self.scaled_width
    }
    pub fn get_scaled_height(&self) -> f64 {
        self.scaled_height
    }
    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.tile_grid.tiles
    }
    pub fn get_graphics(&mut self) -> &mut Vec<Graphic<'a>> {
        &mut self.graphic_content
    }
    pub fn get_profile(&self) -> &profiler::Profile {
        &self.profile
    }
}


