mod blocks;
mod core;
mod items;

use core::world::World;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::time::{Duration, Instant};

fn show(world: &World, ticks: u64) {
    disable_raw_mode().ok();
    println!("\r\n  tick {}\r\n", ticks);
    for line in world.tree_lines() {
        println!("  {}\r", line);
    }
    println!("\r  [d] show storage   [q] quit\r");
    enable_raw_mode().ok();
}

fn run(mut world: World, farm_input: usize) {
    enable_raw_mode().ok();
    println!("farm running — [d] show storage, [q] quit\r");

    let mut ticks = 0u64;
    let mut last_drop = Instant::now();

    loop {
        if event::poll(Duration::ZERO).unwrap_or(false) {
            if let Ok(Event::Key(key)) = event::read() {
                match key.code {
                    KeyCode::Char('d') => show(&world, ticks),
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        if last_drop.elapsed() >= Duration::from_millis(100) {
            world.put_item(farm_input, "gold_ingot", 1);
            world.put_item(farm_input, "rotten_flesh", 1);
            last_drop = Instant::now();
        }

        world.tick();
        ticks += 1;
        std::thread::sleep(Duration::from_millis(50));
    }

    disable_raw_mode().ok();
    show(&world, ticks);
}

fn main() {
    let mut world = World::new();

    // ── blocks ───────────────────────────────────────────────────────────────
    let input = world.add_hopper("Input");
    let sort_gold = world.add_hopper("Sort Gold");
    let chest_gold = world.add_chest("Gold Chest");
    let sort_flesh = world.add_hopper("Sort Rotten Flesh");
    let chest_flesh = world.add_chest("Flesh Chest");
    let leftover = world.add_hopper("Leftover");

    // ── connections ──────────────────────────────────────────────────────────
    world.connect(input, sort_gold);

    world.set_filter(sort_gold, "gold_ingot");
    world.connect(sort_gold, chest_gold);
    world.set_overflow(sort_gold, sort_flesh);

    world.set_filter(sort_flesh, "rotten_flesh");
    world.connect(sort_flesh, chest_flesh);
    world.set_overflow(sort_flesh, leftover);

    // ── run ──────────────────────────────────────────────────────────────────
    run(world, input);
}
