use tile::Tile;

pub trait TileAgent {
    fn update(&mut self, tile: &Tile);
    fn execute(&mut self, tile: &mut Tile);
}