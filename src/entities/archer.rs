use bevy::asset::AssetServer;
use bevy::{ecs::system::Resource, prelude::Handle, render::texture::Image};
use bevy_asset_loader::prelude::*;

use crate::animation::AnimationBundle;

#[derive(AssetCollection, Resource)]
pub struct ArcherBlue {
    #[asset(path = "sprites/Factions/Knights/Troops/Archer/Blue/Archer_Blue.png")]
    pub image: Handle<Image>,
    #[asset(path = "sprites/Factions/Knights/Troops/Archer/Blue/Archer_Blue.animations.json")]
    pub animations: Handle<AnimationBundle>,
}
