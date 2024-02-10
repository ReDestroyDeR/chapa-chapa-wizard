pub mod coordinator;

use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

use crate::helpers::tiled::TiledMapBundle;

pub mod level1;

pub type Vec2<T> = Vec<Vec<T>>;

#[derive(Bundle)]
pub struct LevelBundle {
    pub tilemap: TiledMapBundle,
    pub level: Level,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde[try_from="Vec<Vec<T>>", into="Vec<Vec<T>>"]]
pub struct Grid<T: Debug + Clone> {
    vec2: Vec2<T>,
    y_max: usize,
    x_max: usize,
}

#[derive(Debug)]
pub enum GridError {
    OutOfBounds { asked: usize, max: usize },
    MismatchedWidth { widths: Vec<usize> },
    NoDataInColumns,
}

impl Display for GridError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridError::MismatchedWidth { widths } => write!(f, "MismatchedWidth {:?}", widths),
            GridError::OutOfBounds { asked, max } => {
                write!(f, "OutOfBounds asked: {} max: {}", asked, max)
            }
            GridError::NoDataInColumns => write!(f, "No data in columns"),
        }
    }
}

impl<T: Debug + Clone> From<Grid<T>> for Vec<Vec<T>> {
    fn from(value: Grid<T>) -> Self {
        value.vec2
    }
}

impl<T: Debug + Clone> TryFrom<Vec<Vec<T>>> for Grid<T> {
    type Error = GridError;

    fn try_from(vec2: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let height = vec2.len();
        let widths: Vec<usize> = vec2.iter().map(|row| row.len()).dedup().collect();

        if widths.is_empty() {
            error!("Grid doesn't have any data\n{:#?}", vec2);
            Err(GridError::NoDataInColumns)
        } else if widths.len() > 1 {
            error!("Grid doesn't have constant width\n{:#?}", vec2);
            Err(GridError::MismatchedWidth { widths })
        } else {
            Ok(Self {
                vec2,
                y_max: height,
                x_max: widths[0],
            })
        }
    }
}

impl<T: Debug + Clone> Grid<T> {
    pub fn new(height: usize, width: usize, default: T) -> Self {
        let mut vec2 = Vec::with_capacity(height);

        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(default.clone());
            }
            vec2.push(row);
        }

        Self {
            vec2,
            y_max: height,
            x_max: width,
        }
    }

    pub fn x_max(&self) -> usize {
        self.x_max
    }
    pub fn y_max(&self) -> usize {
        self.y_max
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.vec2
            .as_slice()
            .get(y)
            .and_then(|row| row.as_slice().get(x))
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), GridError> {
        if y > self.vec2.len() {
            Err(GridError::OutOfBounds {
                asked: y,
                max: self.vec2.len(),
            })
        } else {
            let row = &mut self.vec2.as_mut_slice()[y];

            if x > row.len() {
                Err(GridError::OutOfBounds {
                    asked: x,
                    max: row.len(),
                })
            } else {
                row.as_mut_slice()[x] = value;
                Ok(())
            }
        }
    }

    pub fn for_each<F>(&self, mut f: F) -> ()
    where
        F: FnMut(usize, usize, &T) -> (),
    {
        self.vec2
            .iter()
            .enumerate()
            .for_each(|(y, row)| row.iter().enumerate().for_each(|(x, value)| f(x, y, value)))
    }

    pub fn map<R: Debug + Clone>(&self, f: fn(&T) -> R) -> Grid<R> {
        Grid {
            vec2: self
                .vec2
                .iter()
                .map(|row| row.iter().map(f).collect())
                .collect(),
            x_max: self.x_max,
            y_max: self.y_max,
        }
    }

    pub fn search_from_pos(&self, x: usize, y: usize, f: fn(&T) -> bool) -> Vec<(usize, usize)> {
        if self.get(x, y).is_none() {
            warn!("Out of bounds: x {x} y {y}");
            vec![]
        } else {
            if f(&self.vec2[y][x]) {
                return vec![(x, y)];
            }

            let mut ring = Vec::new();
            for d in 1..self.x_max.max(self.y_max) {
                let l = x.saturating_sub(d);
                let r = (x + d).min(self.x_max - 1);
                let u = y.saturating_sub(d);
                let d = (y + d).min(self.y_max - 1);

                // up & down
                for x in l..=r {
                    if let Some(_) = self.get(x, u).filter(|&v| f(v)) {
                        ring.push((x, u));
                    } else if let Some(_) = self.get(x, d).filter(|&v| f(v)) {
                        ring.push((x, d));
                    }
                }

                // left & right
                for y in d - 1..u {
                    if let Some(_) = self.get(l, y).filter(|&v| f(v)) {
                        ring.push((l, y));
                    } else if let Some(_) = self.get(r, y).filter(|&v| f(v)) {
                        ring.push((r, y));
                    }
                }

                if !ring.is_empty() {
                    return ring;
                }
            }
            ring
        }
    }
}

#[derive(Component)]
pub struct Level {
    pub cfg: Handle<LevelConfig>,
}

#[derive(Default, TypeUuid, TypePath, Deserialize, Debug)]
#[uuid = "0b891564-23ca-492a-b03c-816402b496b7"]
pub struct LevelConfig {
    pub tile_size: f32,
    pub walkable_tiles: WalkableTiles,
}

#[derive(Default, Deserialize, Debug)]
#[serde(from = "WalkableTilesDto")]
pub struct WalkableTiles {
    value: Grid<bool>,
}

impl WalkableTiles {
    pub fn nearest_walkable_tiles(&self, current_tile: (i32, i32)) -> Vec<(i32, i32)> {
        let (x, y) = current_tile;

        let x = self.abs_to_local_x(x);
        let y = self.abs_to_local_y(y);
        self.nearest_walkable_tiles_local((x, y))
            .into_iter()
            .map(|(x, y)| (self.local_to_abs_x(x), self.local_to_abs_y(y)))
            .collect()
    }

    pub fn nearest_walkable_tiles_local(
        &self,
        current_tile: (usize, usize),
    ) -> Vec<(usize, usize)> {
        let (x, y) = current_tile;
        if self.is_walkable_local(x, y) {
            vec![current_tile]
        } else {
            self.value.search_from_pos(x, y, |&b| b)
        }
    }

    fn local_to_abs_x(&self, x: usize) -> i32 {
        (x as i32) - (self.value.x_max() / 2) as i32
    }

    fn local_to_abs_y(&self, y: usize) -> i32 {
        (y as i32) - (self.value.y_max() / 2) as i32
    }

    fn abs_to_local_y(&self, y: i32) -> usize {
        let r = y + (self.value.y_max() / 2) as i32;
        if r < 0 {
            0
        } else {
            r as usize
        }
    }

    fn abs_to_local_x(&self, x: i32) -> usize {
        let r = x + (self.value.x_max() / 2) as i32;
        if r < 0 {
            0
        } else {
            r as usize
        }
    }

    pub fn is_walkable_local(&self, x: usize, y: usize) -> bool {
        self.value.get(x, y).map(|n| n.to_owned()).unwrap_or(false)
    }
}

impl From<WalkableTilesDto> for WalkableTiles {
    fn from(dto: WalkableTilesDto) -> Self {
        let mut last_y = 0;
        let mut buf =
            String::with_capacity(dto.value.y_max * dto.value.x_max * 2 + dto.value.y_max);
        dto.value.for_each(|_, y, &i| {
            if y != last_y {
                last_y = y;
                buf.push('\n');
            }
            buf.push((i + b'0') as char);
        });
        info!("Read WalkableTiles:\n{}", buf);

        Self {
            value: dto.value.map(|&i| i == 1),
        }
    }
}

#[derive(Deserialize)]
struct WalkableTilesDto {
    value: Grid<u8>,
}

#[cfg(test)]
mod tests {
    use crate::levels::LevelConfig;

    use super::Grid;

    #[test]
    fn test_level_config_loads() {
        let cfg = std::fs::read_to_string("tests/level/example.ccwl.json").unwrap();
        println!("{:#?}", serde_json::from_str::<LevelConfig>(&cfg).unwrap())
    }

    #[test]
    fn test_spiral_search_works() {
        let mut grid = Grid::new(100, 100, false);
        grid.set(20, 20, true).unwrap();
        assert_eq!(vec!((20, 20)), grid.search_from_pos(60, 60, |&b| b))
    }

    #[test]
    fn test_spiral_finds_ring() {
        let mut grid = Grid::new(100, 100, false);
        grid.set(22, 11, true).unwrap();
        grid.set(23, 11, true).unwrap();
        grid.set(21, 11, true).unwrap();
        grid.set(23, 10, true).unwrap();
        assert_eq!(
            vec!((23, 11), (22, 11), (21, 11)),
            grid.search_from_pos(22, 12, |&b| b)
        )
    }

    #[test]
    fn test_spiral_finds_closest_down_ring() {
        let mut grid = Grid::new(100, 100, false);
        grid.set(21, 3, true).unwrap();
        assert_eq!(vec!((21, 3)), grid.search_from_pos(21, 2, |&b| b))
    }
}
