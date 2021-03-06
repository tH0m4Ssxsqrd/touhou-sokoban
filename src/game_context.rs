use specs::{World, RunNow, WorldExt};
use ggez::{event, Context, GameResult};
use ggez::event::{KeyCode, KeyMods, quit};
use crate::systems::rendering_system::RenderingSystem;
use crate::systems::input_system::InputSystem;
use crate::systems::gameplay_state_system::GameplayStateSystem;
use crate::constant::{RESOURCE_PREFIX_PATH, MAX_LEVEL};
use crate::components::{Position, Direction, Renderable, Wall, Box, Player, Spot, Movable, Blocking, Directional, FloorMaterial, WallType, BoxSpotColor};
use crate::resources::input_queue::InputQueue;
use crate::resources::game_state::GameState;
use crate::resources::sound_library::SoundLibrary;
use crate::resources::level_data::LevelData;
use crate::resources::timer::Timer;
use crate::entity_builder::EntityBuilder;
use std::fs;
use std::cmp::min;
use serde_json::Value;
use ggez::graphics::Color;


pub struct GameContext {
    pub world: World
}

impl GameContext {
    pub fn from(world: World) -> Self {
        GameContext { world }
    }

    pub fn clear(&mut self) {
        self.world.delete_all();
        self.world.write_resource::<GameState>().reset();
        self.world.write_resource::<InputQueue>().clear();
        self.world.write_resource::<LevelData>().reset();
        self.world.write_resource::<SoundLibrary>().clear();
        self.world.write_resource::<Timer>().stop();
    }

    pub fn load_level(&mut self, level: u8) {
        let s = fs::read_to_string(format!("{}/levels/level_{}.json", RESOURCE_PREFIX_PATH, level)).unwrap();
        let level_json: Value = serde_json::from_str(&s).unwrap();

        let mut level_data = self.world.write_resource::<LevelData>();
        level_data.current_level = level;
        level_data.box_spot_identical_mode = level_json["box_spot_identical_mode"].as_bool().unwrap();
        level_data.background_color = Color::from_rgba(
            level_json["background_color"][0].as_u64().unwrap() as u8,
            level_json["background_color"][1].as_u64().unwrap() as u8,
            level_json["background_color"][2].as_u64().unwrap() as u8,
            level_json["background_color"][3].as_u64().unwrap() as u8
        );
        drop(level_data);

        self.generate_map(level_json["map_string"].as_str().unwrap().to_string());
    }

    pub fn initialize_level(&mut self, level: u8, context: &mut Context) {
        self.clear();
        let level = min(level, MAX_LEVEL);

        self.load_level(level);
        self.world.write_resource::<SoundLibrary>().load_music(context, level);
        self.world.write_resource::<Timer>().start();
    }

    pub fn restart_level(&mut self, context: &mut Context) {
        let current_level = self.world.write_resource::<LevelData>().current_level;
        self.initialize_level(current_level, context);
    }

    pub fn register_components(&mut self) {
        self.world.register::<Renderable>();
        self.world.register::<Wall>();
        self.world.register::<Player>();
        self.world.register::<Box>();
        self.world.register::<Spot>();
        self.world.register::<Movable>();
        self.world.register::<Blocking>();
        self.world.register::<Directional>();
    }

    pub fn register_resources(&mut self) {
        self.world.insert(InputQueue::new());
        self.world.insert(GameState::default());
        self.world.insert(SoundLibrary::new());
        self.world.insert(LevelData::default());
        self.world.insert(Timer::new());
    }

    pub fn generate_map(&mut self, map_string: String) {
        let rows = map_string.trim().split('\n').map(|x| x.trim()).collect::<Vec<_>>();

        for (_, &row) in rows.iter().enumerate() {
            let columns = row.split(';').map(|x| x.trim()).collect::<Vec<_>>();
            let entity_data = columns[0].split(' ').collect::<Vec<_>>();

            for (_, &column) in columns.iter().enumerate().filter(|&(i, _)| i != 0) {
                let mut position = Position { x: 0, y: 0, z: 0 };
                for(i, &pos) in column.split(',').collect::<Vec<_>>().iter().enumerate() {
                    match i {
                        0 => position.y = pos.parse::<u8>().unwrap(),
                        1 => position.x = pos.parse::<u8>().unwrap(),
                        _ => panic!("Dimension of position exceeded 2.")
                    }
                }

                match entity_data[0] {
                    "floor" => {
                        let floor_material: FloorMaterial = match entity_data[1] {
                            "grass" => FloorMaterial::Grass,
                            c => panic!("Unrecognized FloorMaterial {}.", c)
                        };
                        EntityBuilder::create_floor(&mut self.world, position, floor_material);
                        let _floor_material: FloorMaterial = match entity_data[1] {
                            "grass" => FloorMaterial::Grass,
                            c => panic!("Unrecognized FloorMaterial {}.", c)
                        };
                    },
                    "wall" => {
                        let wall_type: WallType = match entity_data[1] {
                            "horizontal-end-east" => WallType::Hee,
                            "horizontal-end-west" => WallType::Hew,
                            "horizontal-intersect-north" => WallType::Hin,
                            "horizontal-intersect-south" => WallType::His,
                            "horizontal-wall" => WallType::Hw,
                            "upper-right-corner" => WallType::Urc,
                            "upper-left-corner" => WallType::Ulc,
                            "bottom-right-corner" => WallType::Brc,
                            "bottom-left-corner" => WallType::Blc,
                            "vertical-end-north" => WallType::Ven,
                            "vertical-end-south" => WallType::Ves,
                            "vertical-intersect-east" => WallType::Vie,
                            "vertical-intersect-west" => WallType::Viw,
                            "vertical-wall" => WallType::Vw,
                            c => panic!("Unrecognized WallType {}.", c)
                        };
                        EntityBuilder::create_wall(&mut self.world, position, wall_type);
                        let _wall_type: WallType = match entity_data[1]{
                            "horizontal-end-east" => WallType::Hee,
                            "horizontal-end-west" => WallType::Hew,
                            "horizontal-intersect-north" => WallType::Hin,
                            "horizontal-intersect-south" => WallType::His,
                            "horizontal-wall" => WallType::Hw,
                            "upper-right-corner" => WallType::Urc,
                            "upper-left-corner" => WallType::Ulc,
                            "bottom-right-corner" => WallType::Brc,
                            "bottom-left-corner" => WallType::Blc,
                            "vertical-end-north" => WallType::Ven,
                            "vertical-end-south" => WallType::Ves,
                            "vertical-intersect-east" => WallType::Vie,
                            "vertical-intersect-west" => WallType::Viw,
                            "vertical-wall" => WallType::Vw,
                            c => panic!("Unrecognized WallType {}.", c)
                        };
                    },
                    "box" => {
                    
                        let box_color: BoxSpotColor = match entity_data[1] {
                            "black" => BoxSpotColor::Black,
                            c => panic!("Unrecognized BoxColor {}.", c)
                        };
                        EntityBuilder::create_box(&mut self.world, position, box_color);
                    },
                    "spot" => {
                        let spot_color: BoxSpotColor = match entity_data[1] {
                            "black" => BoxSpotColor::Black,
                            c => panic!("Unrecognized SpotColor {}.", c)
                        };
                        EntityBuilder::create_spot(&mut self.world, position, spot_color);
                    },
                    "char" => {
                        EntityBuilder::create_player(&mut self.world, position, Direction::Left);
                    },
                    c => panic!("Unrecognized map item {}", c)
                }
            }
        }
    }
}

impl event::EventHandler for GameContext {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let mut is = InputSystem::new();
        is.run_now(&self.world);
        drop(is);

        let mut gss = GameplayStateSystem::new();
        gss.run_now(&self.world);
        drop(gss);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut rs = RenderingSystem::from(ctx, self);
        rs.run_now(&self.world);
        drop(rs);

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Escape => quit(ctx),
            KeyCode::R => self.restart_level(ctx),
            _ => {
                let mut input_queue = self.world.write_resource::<InputQueue>();
                input_queue.push(keycode);
            }
        }
    }
}
