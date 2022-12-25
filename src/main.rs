#![feature(portable_simd)]

use std::{
    iter,
    simd::{LaneCount, Simd, SupportedLaneCount},
    time::Instant,
};

fn decode_lanes<const LANES: usize>(
    buffer: &[u64],
    id_bits: usize,
    global_offset: usize,
    entries_per_chunk: usize,
    output: &mut Vec<u64>,
) where
    LaneCount<LANES>: SupportedLaneCount,
{
    let global_offsets = Simd::<usize, LANES>::splat(global_offset);

    let lane_offsets = Simd::<usize, LANES>::from_slice(&[
        0, 1024, 2048, 3072, 4096, 5120, 6144, 7168, 8192, 9216, 10240, 11264, 12288, 13312, 14336,
        15360, 16384, 17408, 18432, 19456, 20480, 21504, 22528, 23552, 24576, 25600, 26624, 27648,
        28672, 29696, 30720, 31744, 32768, 33792, 34816, 35840, 36864, 37888, 38912, 39936, 40960,
        41984, 43008, 44032, 45056, 46080, 47104, 48128, 49152, 50176, 51200, 52224, 53248, 54272,
        55296, 56320, 57344, 58368, 59392, 60416, 61440, 62464, 63488, 64512,
    ]);

    let simd_offsets = global_offsets + lane_offsets;

    for index in 0..entries_per_chunk {
        let byte_offset = (id_bits * index) / 64;
        let bit_offset = (id_bits * index) % 64;

        let high_indexes = simd_offsets + Simd::splat(byte_offset);
        let low_indexes = high_indexes + Simd::splat(1);

        let mut high_bytes = Simd::gather_or_default(buffer, high_indexes);
        high_bytes <<= Simd::splat(bit_offset as u64);
        high_bytes >>= Simd::splat(64 - (id_bits as u64));

        let mut low_bytes = Simd::gather_or_default(buffer, low_indexes);
        low_bytes >>= Simd::splat(id_bits.saturating_sub(64 - bit_offset) as u64);

        let outputs = high_bytes | low_bytes;
        output.extend_from_slice(outputs.as_array());
    }
}

// This assumes `buffer` is already organized in chunks of 64 bits
fn decode(
    buffer: &[u64],
    chunks: usize,
    id_bits: usize,
    message_bytes: usize,
    output: &mut Vec<u64>,
) {
    let entries_per_chunk = 8192 / (id_bits + message_bytes * 8);
    let complete_chunks = chunks - 1;

    let mut global_offset = 0;

    // while global_offset + 1024 * 64 < complete_chunks * 1024 {
    //     decode_lanes::<64>(buffer, id_bits, global_offset, entries_per_chunk, output);
    //     global_offset += 1024 * 64;
    // }

    // while global_offset + 1024 * 32 < complete_chunks * 1024 {
    //     decode_lanes::<32>(buffer, id_bits, global_offset, entries_per_chunk, output);
    //     global_offset += 1024 * 32;
    // }

    // while global_offset + 1024 * 16 < complete_chunks * 1024 {
    //     decode_lanes::<16>(buffer, id_bits, global_offset, entries_per_chunk, output);
    //     global_offset += 1024 * 16;
    // }

    // while global_offset + 1024 * 8 < complete_chunks * 1024 {
    //     decode_lanes::<8>(buffer, id_bits, global_offset, entries_per_chunk, output);
    //     global_offset += 1024 * 8;
    // }

    // while global_offset + 1024 * 4 < complete_chunks * 1024 {
    //     decode_lanes::<4>(buffer, id_bits, global_offset, entries_per_chunk, output);
    //     global_offset += 1024 * 4;
    // }

    // while global_offset + 1024 * 2 < complete_chunks * 1024 {
    //     decode_lanes::<2>(buffer, id_bits, global_offset, entries_per_chunk, output);
    //     global_offset += 1024 * 2;
    // }

    while global_offset + 1024 * 1 < complete_chunks * 1024 {
        decode_lanes::<1>(buffer, id_bits, global_offset, entries_per_chunk, output);
        global_offset += 1024 * 1;
    }
}

fn main() {
    let buffer = iter::repeat_with(rand::random::<u64>)
        .take(197504)
        .collect::<Vec<_>>();

    let start = Instant::now();

    let mut output = Vec::with_capacity(150000);
    decode(buffer.as_slice(), 1543, 32, 8, &mut output);

    println!("Elapsed: {:?}", start.elapsed());
    println!("Decoded: {}", output.len());
}
