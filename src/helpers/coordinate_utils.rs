use bevy::math::Vec2;
use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapSize};

pub trait CoordinateOps {
    fn relative_to(&self, zero: &Self) -> Self;

    fn undo_relative(&self, zero: &Self) -> Self;

    fn abs(&self) -> Self;

    fn copy_signs(&self, other: &Self) -> Self;

    fn tiled_top_left(&self, map_size: &TilemapSize, grid_size: &TilemapGridSize) -> Self;
}

impl CoordinateOps for Vec2 {
    fn relative_to(&self, zero: &Self) -> Self {
        Vec2::new(self.x - zero.x, self.y - zero.y)
    }

    fn undo_relative(&self, zero: &Self) -> Self {
        Vec2::new(self.x + zero.x, self.y + zero.y)
    }

    fn abs(&self) -> Self {
        Vec2::new(self.x.abs(), self.y.abs())
    }

    fn copy_signs(&self, other: &Self) -> Self {
        Vec2::new(self.x.copysign(other.x), self.y.copysign(other.y))
    }

    fn tiled_top_left(&self, map_size: &TilemapSize, grid_size: &TilemapGridSize) -> Self {
        Vec2::new(self.x, self.y + grid_size.y * ((map_size.y - 1) as f32))
    }
}


#[cfg(test)]
mod tests {
    use bevy::math::Vec2;

    use crate::helpers::coordinate_utils::CoordinateOps;

    #[test]
    fn relative_to_should_relativize() {
        let zero = Vec2::new(5., 5.);
        let position = Vec2::new(3., 6.);
        let expected = Vec2::new(-2., 1.);
        assert_eq!(position.relative_to(&zero), expected)
    }

    #[test]
    fn relative_to_should_relativize_with_negatives() {
        let zero = Vec2::new(-100.4, 86.);
        let position = Vec2::new(0., 0.);
        let expected = Vec2::new(100.4, -86.);
        assert_eq!(position.relative_to(&zero), expected)
    }

    #[test]
    fn undo_relative_to_should_relativize() {
        let zero = Vec2::new(5., 5.);
        let relative_positon = Vec2::new(-2., 1.);
        let expected = Vec2::new(3., 6.);
        assert_eq!(relative_positon.undo_relative(&zero), expected)
    }

    #[test]
    fn undo_relative_to_should_relativize_with_negatives() {
        let zero = Vec2::new(-100.4, 86.);
        let relative_positon = Vec2::new(100.4, -86.);
        let expected = Vec2::new(0., 0.);
        assert_eq!(relative_positon.undo_relative(&zero), expected)
    }

    #[test]
    fn abs_inverts_negatives() {
        let position = Vec2::new(-1000., -1.);
        let expected = Vec2::new(1000., 1.);
        assert_eq!(position.abs(), expected)
    }

    #[test]
    fn abs_ignores_positives() {
        let position = Vec2::new(1000., 1.);
        let expected = Vec2::new(1000., 1.);
        assert_eq!(position.abs(), expected)
    }

    #[test]
    fn copy_signs_copies_changed() {
        let position = Vec2::new(1000., 1.);
        let signs = Vec2::new(-4., -3.);
        let expected = Vec2::new(-1000., -1.);
        assert_eq!(position.copy_signs(&signs), expected)
    }

    #[test]
    fn copy_signs_change_x() {
        let position = Vec2::new(1000., 1.);
        let signs = Vec2::new(-8., 12.);
        let expected = Vec2::new(-1000., 1.);
        assert_eq!(position.copy_signs(&signs), expected)
    }

    #[test]
    fn copy_signs_change_y() {
        let position = Vec2::new(1000., 1.);
        let signs = Vec2::new(34., -54.);
        let expected = Vec2::new(1000., -1.);
        assert_eq!(position.copy_signs(&signs), expected)
    }

    #[test]
    fn copy_signs_do_nothing_when_match() {
        let position = Vec2::new(1000., 1.);
        let signs = Vec2::new(3., 2.);
        let expected = Vec2::new(-1000., 1.);
        assert_eq!(position.copy_signs(&signs), expected)
    }
}
