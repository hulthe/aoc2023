#![feature(test)]
#![feature(iter_array_chunks, array_chunks, array_windows)]
#![feature(iter_advance_by, slice_partition_dedup)]
#![feature(binary_heap_drain_sorted, btree_cursors)]
extern crate test;

mod util;

aoc_macro::generate_days!(2023);
