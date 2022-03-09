use rust_sc2::prelude::*;

mod unit_spawner;
// TODO: Pull request for Actions bot.rs and action.rs

enum Bots<'a> {
    UnitSpawner(&'a mut unit_spawner::bots::UnitSpawner),
    UnitSpawnerMover(&'a mut unit_spawner::bots::UnitSpawnerMover),
}

fn start_the_runner<'a>(bot: &'a mut impl Player) {}

// Example of how to use runner
fn main() -> SC2Result<()> {
    let mut unit_spawner_bot = unit_spawner::bots::UnitSpawner::default();
    let mut unit_spawner_mover_bot = unit_spawner::bots::UnitSpawnerMover::default();

    let mut vec_of_bots = Vec::<Bots>::new();
    vec_of_bots.push(Bots::UnitSpawner(&mut unit_spawner_bot));
    vec_of_bots.push(Bots::UnitSpawnerMover(&mut unit_spawner_mover_bot));

    for bot in vec_of_bots {
        start_the_runner(&mut bot);
    }

    // Bot vs Computer
    // 1. Initialize runner
    for i in 0..2 {
        if i == 0 {
            let mut runner = RunnerSingle::new(
                &mut unit_spawner_bot,
                Computer::new(Race::Random, Difficulty::VeryEasy, Some(AIBuild::Macro)),
                "Flat128",
                None, // Client version can be specified, otherwise will be used latest available version
            );
            // 2. Configure runner:
            // runner.set_map("Flat128");
            runner.realtime = false;
            let replay_name = format!("test_replay_{}.SC2Replay", i);
            runner.save_replay_as = Some(&replay_name);

            // 3. Launch SC2:
            runner.launch()?;

            // 4. Run games
            // Run game once:
            runner.run_game()?;
        } else if i == 1 {
            let mut runner = RunnerSingle::new(
                &mut unit_spawner_mover_bot,
                Computer::new(Race::Random, Difficulty::VeryEasy, Some(AIBuild::Macro)),
                "Flat128",
                None, // Client version can be specified, otherwise will be used latest available version
            );
            // 2. Configure runner:
            // runner.set_map("Flat128");
            runner.realtime = false;
            let replay_name = format!("test_replay_{}.SC2Replay", i);
            runner.save_replay_as = Some(&replay_name);

            // 3. Launch SC2:
            runner.launch()?;

            // 4. Run games
            // Run game once:
            runner.run_game()?;
        }
    }

    Ok(())
}
