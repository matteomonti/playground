use madvise::{madvise, AccessPattern};
use memmap::MmapOptions;
use std::{fs::File, hint::black_box, iter, os::unix::prelude::MetadataExt, time::Instant};

fn gather(bytes: &[u8], block_size: usize, reads: &[usize]) -> Vec<u8> {
    let mut gathered = Vec::with_capacity(reads.len() * block_size);

    for read in reads {
        unsafe {
            madvise(
                bytes.as_ptr().offset((read * block_size) as isize),
                block_size,
                AccessPattern::WillNeed,
            )
            .unwrap();
        }
    }

    for read in reads {
        gathered.extend_from_slice(&bytes[(read * block_size)..((read + 1) * block_size)]);
    }

    gathered
}

fn main() {
    let file = File::open("/home/ubuntu/blob.bin").unwrap();
    let metadata = file.metadata().unwrap();

    let length = metadata.len() as usize;
    let block_size = metadata.blksize() as usize * 2;
    let blocks = length / block_size;

    println!("Block size: {block_size}");
    println!("Number of blocks: {blocks}");

    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };

    unsafe {
        madvise(mmap.as_ptr(), length, AccessPattern::Random).unwrap();
    }

    let reads = iter::repeat_with(|| rand::random::<usize>() % blocks)
        .take(32000)
        .collect::<Vec<_>>();

    let start = Instant::now();
    let _gathered = black_box(gather(&mmap, block_size, reads.as_slice()));
    println!("Elapsed: {:?}", start.elapsed());
}
