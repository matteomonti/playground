use rand::RngCore;
use std::time::Instant;

const PAYLOADS: usize = 131072;
const PAYLOAD_SIZE: usize = 12;

const BATCH_SIZE: usize = PAYLOADS * PAYLOAD_SIZE;
const USABLE_CHUNK_SIZE: usize = (1024 / PAYLOAD_SIZE) * PAYLOAD_SIZE;
const CHUNKS: usize = (BATCH_SIZE + USABLE_CHUNK_SIZE - 1) / USABLE_CHUNK_SIZE;

fn main() {
    loop {
        let mut batch = [0u8; BATCH_SIZE];
        rand::thread_rng().fill_bytes(batch.as_mut_slice());

        let mut buffer = [0u8; 1024 * CHUNKS];

        let start = Instant::now();

        for (index, chunk) in batch.chunks(USABLE_CHUNK_SIZE).enumerate() {
            buffer[(index * 1024)..(index * 1024 + chunk.len())].copy_from_slice(chunk);
        }

        let hash = blake3::hash(buffer.as_slice());

        println!("Elapsed: {:?}", start.elapsed());
        println!("Hash: {}", hash);
    }
}
