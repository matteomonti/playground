use rand::RngCore;
use std::time::Instant;

fn ids(
    buffer: &[u8],
    ids_per_chunk: usize,
    id_bytes: usize,
    suffix_bits: u64,
    suffix: u64,
) -> Vec<u64> {
    let mut output = Vec::with_capacity(150000);

    for chunk in buffer.chunks(1024) {
        for prefix in chunk.chunks(id_bytes).take(ids_per_chunk) {
            let mut buffer = [0u8; 8];
            buffer[0..id_bytes].copy_from_slice(prefix);

            let id = u64::from_le_bytes(buffer);
            let id = (id << suffix_bits) | suffix;

            output.push(id);
        }
    }

    output
}

fn main() {
    loop {
        let buffers = (0..1000)
            .map(|_| {
                let mut buffer = vec![0u8; 1580032];
                rand::thread_rng().fill_bytes(buffer.as_mut_slice());
                buffer
            })
            .collect::<Vec<_>>();

        let start = Instant::now();

        for buffer in buffers {
            let ids = ids(buffer.as_slice(), 85, 3, 11, 574);
            
            if rand::random::<u64>() == 0 {
                println!("Len: {}", ids.len());
                println!("Sum: {:?}", ids.into_iter().sum::<u64>());
            }
        }

        println!("Elapsed: {:?}", start.elapsed());
    }
}
