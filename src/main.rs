use rust_sc2::{bot::Bot, prelude::*};
use std::ops::DerefMut;
mod unit_spawner;
// TODO: Pull request for Actions bot.rs and action.rs

enum Bots {
    UnitSpawner(unit_spawner::bots::UnitSpawner),
    UnitSpawnerMover(unit_spawner::bots::UnitSpawnerMover),
}

fn start_the_runner(mut bot: impl Player + DerefMut<Target = Bot>, name: &str) -> SC2Result<()> {
    let mut runner = RunnerSingle::new(
        &mut bot,
        Computer::new(Race::Random, Difficulty::VeryEasy, Some(AIBuild::Macro)),
        "Flat128",
        None, // Client version can be specified, otherwise will be used latest available version
    );
    // 2. Configure runner:
    // runner.set_map("Flat128");
    runner.realtime = false;
    let replay_name = format!("test_replay_{}.SC2Replay", name);
    runner.save_replay_as = Some(&replay_name);
    // 3. Launch SC2:
    runner.launch()?;
    // 4. Run games
    // Run game once:
    runner.run_game()?;
    Ok(())
}
// Example of how to use runner
fn main() -> SC2Result<()> {
    let unit_spawner_bot = unit_spawner::bots::UnitSpawner::default();
    let unit_spawner_mover_bot = unit_spawner::bots::UnitSpawnerMover::default();

    let mut vec_of_bots = Vec::<Bots>::new();
    vec_of_bots.push(Bots::UnitSpawner(unit_spawner_bot));
    vec_of_bots.push(Bots::UnitSpawnerMover(unit_spawner_mover_bot));

    for bot in vec_of_bots {
        match bot {
            Bots::UnitSpawner(spawner) => start_the_runner(spawner, "spawner")?,
            Bots::UnitSpawnerMover(mover) => start_the_runner(mover, "mover")?,
        }
    }

    Ok(())
}
