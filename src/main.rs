use memmap::MmapOptions;
use std::{fs::File, hint::black_box, iter, os::unix::prelude::MetadataExt, time::Instant};

fn gather(bytes: &[u8], block_size: usize, reads: &[usize]) -> Vec<u8> {
    let mut gathered = Vec::with_capacity(reads.len() * block_size);

    for read in reads {
        gathered.extend_from_slice(&bytes[(read * block_size)..((read + 1) * block_size)]);
    }

    gathered
}

fn main() {
    let file = File::open("/home/mmonti/blob.bin").unwrap();
    let metadata = file.metadata().unwrap();

    let block_size = metadata.blksize() as usize;
    let blocks = metadata.len() as usize / block_size;

    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let reads = iter::repeat_with(|| rand::random::<usize>() % blocks)
        .take(32000)
        .collect::<Vec<_>>();

    let start = Instant::now();
    let _gathered = black_box(gather(&mmap, block_size, reads.as_slice()));
    println!("Elapsed: {:?}", start.elapsed());
}
