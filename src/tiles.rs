use bevy::prelude::Color;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Tiles {
    Sand,
    Grass,
    Water,
    Forest,
}

impl Tiles {
    pub fn values() -> Vec<Tiles> {
        vec![Tiles::Sand, Tiles::Grass, Tiles::Water, Tiles::Forest]
    }

    pub fn allowed_neighbors(&self) -> Vec<Tiles> {
        match self {
            Tiles::Sand => vec![Tiles::Grass, Tiles::Water],
            Tiles::Grass => vec![Tiles::Sand, Tiles::Forest],
            Tiles::Water => vec![Tiles::Sand],
            Tiles::Forest => vec![Tiles::Grass],
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Tiles::Sand => Color::rgb(0.9, 0.9, 0.9),
            Tiles::Grass => Color::rgb(0.0, 0.5, 0.0),
            Tiles::Water => Color::rgb(0.0, 0.0, 0.5),
            Tiles::Forest => Color::rgb(0.0, 0.5, 0.0),
        }
    }

    pub fn values_len() -> usize {
        4
    }
}
