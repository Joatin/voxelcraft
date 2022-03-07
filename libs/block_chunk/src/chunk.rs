use std::mem;
use bincode::config;
use std::error::Error;
use flate2::read::GzDecoder;
use std::io::Read;
use flate2::write::GzEncoder;
use flate2::Compression;
use crate::BlockOffset;

#[derive(Clone, Debug, bincode::Encode, bincode::Decode)]
pub struct Chunk<T: 'static, const SIZE: usize> {
    blocks: [[[T; SIZE]; SIZE]; SIZE]
}

impl<T: 'static + Default + Copy, const SIZE: usize> Chunk<T, SIZE> {
    pub fn new() -> Self {
        Self {
            blocks: [[[T::default(); SIZE]; SIZE]; SIZE]
        }
    }
}


impl<T: 'static, const SIZE: usize> Chunk<T, SIZE> {


    pub fn get(&self, position: &BlockOffset<SIZE>) -> &T {

        assert!(position.x < SIZE);
        assert!(position.y < SIZE);
        assert!(position.z < SIZE);

        &self.blocks[position.z][position.y][position.x]
    }

    pub fn set(&mut self, mut value: T, position: &BlockOffset<SIZE>) -> T {
        assert!(position.x < SIZE);
        assert!(position.y < SIZE);
        assert!(position.z < SIZE);

        mem::swap( &mut self.blocks[position.z][position.y][position.x], &mut value);
        value
    }
}

impl<T: 'static + Clone + Copy, const SIZE: usize> Chunk<T, SIZE> {

    pub fn new_checker(val_1: T, val_2: T) -> Self {
        let mut blocks = [[[val_1; SIZE]; SIZE]; SIZE];
        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    if z % 2 == 1 && y % 2 == 1 && x % 2 == 1 {
                        blocks[z][y][x] = val_2.clone()
                    }
                }
            }
        }

        Self {
            blocks
        }
    }
}



impl<T: 'static + bincode::Encode, const SIZE: usize> Chunk<T, SIZE> {
    pub fn compress(self) -> Result<Vec<u8>, Box<dyn Error>> {
        let encoded = bincode::encode_to_vec(self, config::standard())?;
        let encoder = GzEncoder::new(encoded, Compression::best());
        Ok(encoder.finish()?)
    }
}

impl<T: 'static + bincode::Decode, const SIZE: usize> Chunk<T, SIZE> {
    pub fn from_compressed(compressed_bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        let mut decoder = GzDecoder::new(compressed_bytes);
        let mut decompressed = vec![];
        decoder.read(&mut decompressed)?;
        let (chunk, _) = bincode::decode_from_slice(&decompressed, config::standard())?;
        Ok(chunk)
    }
}

impl<T: 'static + Default + Copy, const SIZE: usize> Default for Chunk<T, SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::Chunk;

    #[test]
    fn it_should_get_and_set() {
        let mut chunk = Chunk::<usize, 4>::new();

        chunk.set(100, &(2, 2, 2).into());

        assert_eq!(*chunk.get(&(2, 2, 2).into()), 100);
    }
}