use crate::blocks::block::{AnyBlock, Container};
use crate::blocks::chest::Chest;
use crate::blocks::hopper::Hopper;
use crate::items::item::{Item, ItemStack};
use std::collections::HashSet;

pub struct World {
    blocks: Vec<AnyBlock>,
    names: Vec<String>,
}

impl World {
    pub fn new() -> Self {
        World {
            blocks: Vec::new(),
            names: Vec::new(),
        }
    }

    pub fn add_hopper(&mut self, name: &str) -> usize {
        self.blocks.push(AnyBlock::Hopper(Hopper::new()));
        self.names.push(name.to_string());
        self.blocks.len() - 1
    }

    pub fn add_chest(&mut self, name: &str) -> usize {
        self.blocks.push(AnyBlock::Chest(Chest::new()));
        self.names.push(name.to_string());
        self.blocks.len() - 1
    }

    pub fn connect(&mut self, from: usize, to: usize) {
        if let Some(h) = self.blocks[from].as_hopper_mut() {
            h.output_id = Some(to);
        }
    }

    pub fn set_overflow(&mut self, hopper: usize, to: usize) {
        if let Some(h) = self.blocks[hopper].as_hopper_mut() {
            h.overflow_id = Some(to);
        }
    }

    pub fn set_filter(&mut self, hopper: usize, item: &str) {
        if let Some(h) = self.blocks[hopper].as_hopper_mut() {
            h.filter = Some(item.to_string());
        }
    }

    pub fn power(&mut self, hopper: usize, on: bool) {
        if let Some(h) = self.blocks[hopper].as_hopper_mut() {
            h.powered = on;
        }
    }

    pub fn put_item(&mut self, block: usize, name: &str, amount: u8) {
        let item = ItemStack { item: Item { name: name.to_string() }, amount };
        self.blocks[block].as_container_mut().inventory_mut().insert(item).ok();
    }

    pub fn count(&self, block: usize, item: &str) -> u32 {
        self.blocks[block].as_container().inventory().total(item)
    }

    pub fn tree_lines(&self) -> Vec<String> {
        let mut has_incoming = HashSet::new();
        for block in &self.blocks {
            if let AnyBlock::Hopper(h) = block {
                if let Some(id) = h.output_id   { has_incoming.insert(id); }
                if let Some(id) = h.overflow_id  { has_incoming.insert(id); }
            }
        }

        let mut lines = Vec::new();
        let mut visited = HashSet::new();

        for i in 0..self.blocks.len() {
            if !has_incoming.contains(&i) {
                lines.push(self.block_label(i));
                visited.insert(i);
                self.collect_children(i, "", &mut lines, &mut visited);
            }
        }

        lines
    }

    pub fn tick(&mut self) {
        let mut planned: Vec<(usize, usize, String)> = Vec::new();

        for i in 0..self.blocks.len() {
            let (output, overflow, powered, filter) = match &self.blocks[i] {
                AnyBlock::Hopper(h) => (h.output_id, h.overflow_id, h.powered, h.filter.clone()),
                _ => continue,
            };

            if powered { continue; }

            let top = match self.blocks[i].as_container().inventory().peek_first_name() {
                Some(n) => n.to_string(),
                None    => continue,
            };

            let destination = match &filter {
                Some(f) if top == *f => output,
                Some(_)              => overflow,
                None                 => output,
            };

            if let Some(dest) = destination {
                if dest != i {
                    planned.push((i, dest, top));
                }
            }
        }

        for (from, to, item_name) in planned {
            let (src, dst) = if from < to {
                let (left, right) = self.blocks.split_at_mut(to);
                (&mut left[from], &mut right[0])
            } else {
                let (left, right) = self.blocks.split_at_mut(from);
                (&mut right[0], &mut left[to])
            };

            if let Some(item) = src.as_container_mut().inventory_mut().take_one(&item_name) {
                if dst.as_container_mut().inventory_mut().insert(item.clone()).is_err() {
                    src.as_container_mut().inventory_mut().insert(item).ok();
                }
            }
        }
    }

    fn block_label(&self, id: usize) -> String {
        let kind = match &self.blocks[id] {
            AnyBlock::Hopper(_) => "Hopper",
            AnyBlock::Chest(_)  => "Chest",
        };
        let contents = self.blocks[id].as_container().inventory().contents_line();

        if contents.is_empty() {
            kind.to_string()
        } else {
            format!("{}  —  {}", kind, contents)
        }
    }

    fn collect_children(
        &self,
        id: usize,
        prefix: &str,
        lines: &mut Vec<String>,
        visited: &mut HashSet<usize>,
    ) {
        let (output, overflow) = match &self.blocks[id] {
            AnyBlock::Hopper(h) => (h.output_id, h.overflow_id),
            _ => (None, None),
        };

        let children: Vec<usize> = [output, overflow]
            .iter()
            .filter_map(|x| *x)
            .collect();

        for (i, child_id) in children.iter().enumerate() {
            if visited.contains(child_id) { continue; }
            visited.insert(*child_id);

            let is_last  = i == children.len() - 1;
            let branch   = if is_last { "└─►" } else { "├─►" };
            lines.push(format!("{}{} {}", prefix, branch, self.block_label(*child_id)));

            let next_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            self.collect_children(*child_id, &next_prefix, lines, visited);
        }
    }
}
