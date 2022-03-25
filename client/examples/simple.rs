use packs_test::TestModPack;
use voxelcraft_client::setup_voxelcraft;

fn main() {
    setup_voxelcraft(TestModPack::new())
}
