use hecs::World;
use tetra::{
    input::{get_keys_down, Key},
    math::Vec2,
    Context,
};

use crate::{map::Map, Player, Position};

pub fn move_player_system(world: &mut World, ctx: &Context) {
    let mut binding = world.query::<(&Player, &mut Position)>();
    let (_, (_, Position(pos))) = binding.into_iter().next().unwrap();
    let mut binding = world.query::<(&mut Map,)>();
    let (_, (map,)) = binding.into_iter().next().unwrap();
    for key in get_keys_down(&ctx) {
        let mut step = *pos;
        match key {
            Key::W => {
                step += Vec2::new(0, -1);
            }
            Key::S => {
                step += Vec2::new(0, 1);
            }
            Key::A => {
                step += Vec2::new(-1, 0);
            }
            Key::D => {
                step += Vec2::new(1, 0);
            }
            _ => {}
        }
        if !map.obstacles[map.xy_index(step.x, step.y)] {
            println!("x {} y {}", step.x, step.y);
            *pos = step;
        }
    }
}
