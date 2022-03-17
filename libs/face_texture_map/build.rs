use mipmap::generate_mipmap_for_path_and_save_to_out_dir;
use mipmap::image::imageops::FilterType;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=assets/");
    println!("cargo:rerun-if-changed=build.rs");

    generate_mipmap_for_path_and_save_to_out_dir("assets", FilterType::Gaussian).unwrap();
}
