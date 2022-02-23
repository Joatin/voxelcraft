use voxelcraft_client::setup_voxelcraft;
use packs_test::TestModPack;

#[tokio::main]
async fn main() {
    setup_voxelcraft(TestModPack::new()).await
}