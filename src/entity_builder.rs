use specs::{World, WorldExt, Builder};
use crate::constant::{WALL_Z, FLOOR_Z, BOX_Z, SPOT_Z, PLAYER_Z};
use crate::components::*;
use std::collections::HashMap;


pub struct EntityBuilder;

impl EntityBuilder {
    pub fn create_wall(world: &mut World, position: Position, walltype: WallType) {
        world.create_entity()
            .with(Renderable::from(
                Position { z: WALL_Z, ..position },
                "/images/wall_{wall_type}.png",
                vec![
                    (String::from("wall_type"), walltype.to_string())
                ].into_iter().collect()))
            .with(Wall::new())
            .with(Blocking::new())
            .build();
    }

    pub fn create_floor(world: &mut World, position: Position, material: FloorMaterial) {
        world.create_entity()
            .with(Renderable::from(
                Position { z: FLOOR_Z, ..position },
                "/images/floor_{floor_material}.png",
                  vec![
                      (String::from("floor_material"), material.to_string())
                  ].into_iter().collect()))
            .build();
    }

    pub fn create_box(world: &mut World, position: Position, color: BoxSpotColor) {
        world.create_entity()
            .with(Renderable::from(
                Position { z: BOX_Z, ..position },
                "/images/box_{box_color}.png",
                vec![
                      (String::from("box_color"), color.to_string())
                ].into_iter().collect()))
            .with(Box::new())
            .with(Blocking::new())
            .with(Movable::new())
            .build();
    }

    pub fn create_spot(world: &mut World, position: Position, color: BoxSpotColor) {
        world.create_entity()
            .with(Renderable::from(
                Position { z: SPOT_Z, ..position },
                "/images/spot_{spot_color}.png",
                vec![
                    (String::from("spot_color"), color.to_string())
                ].into_iter().collect()))
            .with(Spot::new())
            .build();
    }

    pub fn create_player(world: &mut World, position: Position, direction: Direction) {
        world.create_entity()
            .with(Renderable::from(
                Position { z: PLAYER_Z, ..position },
                "/images/char_down_1.png",
                HashMap::new()))
            .with(Player::new())
            .with(Movable::new())
            .with(Directional::from(direction))
            .build();
    }
}
