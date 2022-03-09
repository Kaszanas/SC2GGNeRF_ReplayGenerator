use rust_sc2::prelude::*;

use crate::unit_spawner::utils;

#[bot]
#[derive(Default)]
pub struct UnitSpawner {
    pub vector_of_units: Vec<UnitTypeId>,
    pub is_clean: bool,
    pub current_loop: u32,
    pub base_structure_id: u64,
}
impl Player for UnitSpawner {
    fn get_player_settings(&self) -> PlayerSettings {
        PlayerSettings::new(Race::Protoss)
    }
    fn on_start(&mut self) -> SC2Result<()> {
        // Hold position on all of the workers:
        for worker in &self.units.my.workers {
            worker.hold_position(false);
        }
        self.debug.cheat_minerals();

        // Getting a vector of units that will be spawned:
        self.vector_of_units = utils::get_units_and_structures();
        // Killing all units that belong to the bot
        // This ommits the base_structure in order not to finishe the game immediately:
        self.base_structure_id = self.units.my.townhalls.first().unwrap().tag();
        let units = self
            .units
            .my
            .all
            .iter()
            .filter(|u| u.tag() != self.base_structure_id)
            .map(|u| u.tag())
            .collect::<Vec<u64>>();

        self.debug.kill_units(units.iter());
        self.is_clean = true;
        self.current_loop = self.state.observation.game_loop();

        // Revealing the whole map:
        self.debug.show_map();
        Ok(())
    }

    // Called on every game step
    fn on_step(&mut self, iteration: usize) -> SC2Result<()> {
        let current_loop = self.state.observation.game_loop();

        // If all of the units were spawned and the map is clean,
        // we finish the game:
        if self.vector_of_units.is_empty() && self.is_clean {
            self.debug.end_game();
            return Ok(());
        }

        // Safe guard against duplicate runs of on_step
        // This is in case that on_step is called twice for the same game_loop:
        if current_loop == self.current_loop {
            return Ok(());
        }
        self.current_loop = current_loop;

        let gameloop_interval: u32 = 100;
        // Calculate the center of the map:
        let map_size = self.game_info.map_size;
        let center_x = map_size.x as f32 * 0.5;
        let center_y = map_size.y as f32 * 0.5;

        // Spawning or cleaning units in a given interval:
        if self.state.observation.game_loop() % gameloop_interval == 0 {
            if self.is_clean {
                if let Some(unit_type) = self.vector_of_units.pop() {
                    let number_of_units: u32 = 1;
                    let bot_id = self.player_id.to_owned();

                    // Spawn units:
                    self.debug.create_units([&(
                        unit_type,
                        Some(bot_id),
                        Point2::new(center_x, center_y),
                        number_of_units,
                    )]);
                    self.is_clean = false;
                }
            } else {
                // Killing all units that belong to the bot
                // This ommits the base_structure in order not to finishe the game immediately:
                let units = self
                    .units
                    .my
                    .all
                    .iter()
                    .filter(|u| u.tag() != self.base_structure_id)
                    .map(|u| u.tag())
                    .collect::<Vec<u64>>();

                self.debug.kill_units(units.iter());
                self.is_clean = true;
            }
        }

        Ok(())
    }
}

#[bot]
#[derive(Default)]
pub struct UnitSpawnerMover {
    pub vector_of_units: Vec<UnitTypeId>,
    pub is_clean: bool,
    pub current_loop: u32,
    pub base_structure_id: u64,
}
impl Player for UnitSpawnerMover {
    fn get_player_settings(&self) -> PlayerSettings {
        PlayerSettings::new(Race::Protoss)
    }
    fn on_start(&mut self) -> SC2Result<()> {
        // Hold position on all of the workers:
        for worker in &self.units.my.workers {
            worker.hold_position(false);
        }
        self.debug.cheat_minerals();

        // Getting a vector of units that will be spawned:
        self.vector_of_units = utils::get_units_and_structures();
        // Killing all units that belong to the bot
        // This ommits the base_structure in order not to finishe the game immediately:
        self.base_structure_id = self.units.my.townhalls.first().unwrap().tag();
        let units = self
            .units
            .my
            .all
            .iter()
            .filter(|u| u.tag() != self.base_structure_id)
            .map(|u| u.tag())
            .collect::<Vec<u64>>();

        self.debug.kill_units(units.iter());
        self.is_clean = true;
        self.current_loop = self.state.observation.game_loop();

        // Revealing the whole map:
        self.debug.show_map();
        Ok(())
    }

    // Called on every game step
    fn on_step(&mut self, iteration: usize) -> SC2Result<()> {
        let current_loop = self.state.observation.game_loop();

        // If all of the units were spawned and the map is clean,
        // we finish the game:
        if self.vector_of_units.is_empty() && self.is_clean {
            self.debug.end_game();
            return Ok(());
        }

        // Safe guard against duplicate runs of on_step
        // This is in case that on_step is called twice for the same game_loop:
        if current_loop == self.current_loop {
            return Ok(());
        }
        self.current_loop = current_loop;

        let gameloop_interval: u32 = 100;
        // Calculate the center of the map:
        let map_size = self.game_info.map_size;
        let center_x = map_size.x as f32 * 0.5;
        let center_y = map_size.y as f32 * 0.5;
        let half_gameloop_interval = gameloop_interval / 2;
        if self.state.observation.game_loop() % half_gameloop_interval == 0
            && self.is_clean != true
            && !(self.state.observation.game_loop() % gameloop_interval == 0)
        {
            println!("Checked that the unit should be moved!");

            // Select the unit in the center
            if let Some(unit) = self.units.my.all.closest(Point2::new(center_x, center_y)) {
                println!("Selected unit that is going to be moved!");

                // Getting the unit facing position and calculating its opposite angle:
                let unit_faces_angle = unit.facing();
                // let oposite_angle = (unit_faces_angle + 180.0) % 360.0;
                let move_by = 2;
                let move_by_x = move_by as f32 * unit_faces_angle.cos();
                let move_by_y = move_by as f32 * unit_faces_angle.sin();

                let target_x = center_x - move_by_x;
                let target_y = center_y - move_by_y;

                println!("Target selection point x:{}, y:{}", center_x, center_y);
                println!("Target difference in x:{}, y:{}", move_by_x, move_by_y);
                println!("Target move point x:{}, y:{}", target_x, target_y);

                // click in the opposite direction by the smallest increment so that it changes the direction
                let move_to_point = Point2::new(target_x, target_y);
                let move_to_target = Target::Pos(move_to_point);
                unit.move_to(move_to_target, false);
                println!("I am supposed to move the unit!");
            };
        }

        // Spawning or cleaning units in a given interval:
        if self.state.observation.game_loop() % gameloop_interval == 0 {
            if self.is_clean {
                if let Some(unit_type) = self.vector_of_units.pop() {
                    let number_of_units: u32 = 1;
                    let bot_id = self.player_id.to_owned();

                    // Spawn units:
                    self.debug.create_units([&(
                        unit_type,
                        Some(bot_id),
                        Point2::new(center_x, center_y),
                        number_of_units,
                    )]);
                    self.is_clean = false;
                }
            } else {
                // Killing all units that belong to the bot
                // This ommits the base_structure in order not to finishe the game immediately:
                let units = self
                    .units
                    .my
                    .all
                    .iter()
                    .filter(|u| u.tag() != self.base_structure_id)
                    .map(|u| u.tag())
                    .collect::<Vec<u64>>();

                self.debug.kill_units(units.iter());
                self.is_clean = true;
            }
        }

        Ok(())
    }
}
