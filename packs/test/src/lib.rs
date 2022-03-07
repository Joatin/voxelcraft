use std::sync::Arc;
use voxelcraft_mod::{Mod, ModPack};

pub struct TestModPack {
    mods: Vec<Arc<dyn Mod>>,
}

impl TestModPack {
    pub fn new() -> Self {
        Self { mods: vec![] }
    }
}

impl ModPack for TestModPack {
    fn mods(&self) -> &[Arc<dyn Mod>] {
        &self.mods
    }
}
