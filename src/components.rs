#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub enum HoubaType {
    Light, // (HoubaLight),      // Experimenting with ways to represent various forms  of entities.
    Mario,
    Home,
    Grumpy,
    Psych,
}
impl Eq for HoubaType {}

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub enum TileType {
    Triangle,
    Hexagon,
    Square,
    Player,
    Cake,
}
impl Eq for TileType {}

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub enum FontType {
    Main,
    MainBold,
}
impl Eq for FontType {}
