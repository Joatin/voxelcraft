use crate::BlockOffset;
use bincode::config;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::error::Error;
use std::io::Read;
use std::mem;

#[derive(Clone, Debug, bincode::Encode, bincode::Decode)]
pub struct Chunk<T: 'static + Send + Sync, const SIZE: usize> {
    blocks: Box<[[[T; SIZE]; SIZE]; SIZE]>,
}

impl<T: 'static + Send + Sync + Default + Copy, const SIZE: usize> Chunk<T, SIZE> {
    pub fn new() -> Self {
        Self {
            blocks: Box::new([[[T::default(); SIZE]; SIZE]; SIZE]),
        }
    }
}

impl<T: 'static + Send + Sync, const SIZE: usize> Chunk<T, SIZE> {
    /// # Panics
    #[inline]
    pub fn get(&self, position: &BlockOffset<SIZE>) -> &T {
        assert!(position.x < SIZE);
        assert!(position.y < SIZE);
        assert!(position.z < SIZE);

        &self.blocks[position.z][position.y][position.x]
    }

    /// # Panics
    #[inline]
    pub fn set(&mut self, mut value: T, position: &BlockOffset<SIZE>) -> T {
        assert!(position.x < SIZE);
        assert!(position.y < SIZE);
        assert!(position.z < SIZE);

        mem::swap(
            &mut self.blocks[position.z][position.y][position.x],
            &mut value,
        );
        value
    }
}

impl<T: 'static + Send + Sync + Clone + Copy, const SIZE: usize> Chunk<T, SIZE> {
    pub fn new_checker(val_1: T, val_2: T) -> Self {
        let mut blocks = Box::new([[[val_1; SIZE]; SIZE]; SIZE]);
        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    if (z + ((y + (x % 2)) % 2)) % 2 == 1 {
                        blocks[z][y][x] = val_2.clone();
                    }
                }
            }
        }

        Self { blocks }
    }

    pub fn set_all(&mut self, value: T) {
        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    self.blocks[z][y][x] = value
                }
            }
        }
    }
}

impl<T: 'static + Send + Sync + bincode::Encode, const SIZE: usize> Chunk<T, SIZE> {
    pub fn compress(self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let encoded = bincode::encode_to_vec(self, config::standard())?;
        let encoder = GzEncoder::new(encoded, Compression::best());
        Ok(encoder.finish()?)
    }
}

impl<T: 'static + Send + Sync + bincode::Decode, const SIZE: usize> Chunk<T, SIZE> {
    pub fn from_compressed(compressed_bytes: &[u8]) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let mut decoder = GzDecoder::new(compressed_bytes);
        let mut decompressed = vec![];
        decoder.read(&mut decompressed)?;
        let (chunk, _) = bincode::decode_from_slice(&decompressed, config::standard())?;
        Ok(chunk)
    }
}

impl<T: 'static + Send + Sync + Default + Copy, const SIZE: usize> Default for Chunk<T, SIZE> {
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

    #[test]
    fn checker_should_be_correct() {
        let chunk: Chunk<_, 4> = Chunk::new_checker(0, 1);

        assert_eq!(chunk.blocks[0][0][0], 0);
        assert_eq!(chunk.blocks[1][0][0], 1);
        assert_eq!(chunk.blocks[0][1][0], 1);
        assert_eq!(chunk.blocks[1][1][0], 0);

        assert_eq!(chunk.blocks[0][0][1], 1);
        assert_eq!(chunk.blocks[1][0][1], 0);
        assert_eq!(chunk.blocks[0][1][1], 0);
        assert_eq!(chunk.blocks[1][1][1], 1);
    }
}
