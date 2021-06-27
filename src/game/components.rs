#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub enum TileType {
    Triangle,
    Hexagon,
    Square,
    Player,
    Cake,
}
impl Eq for TileType {}

