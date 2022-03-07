use voxelcraft_client::setup_voxelcraft;
use packs_test::TestModPack;

fn main() {
    setup_voxelcraft(TestModPack::new())
}