#![feature(test)]

extern crate test;

use std::collections::BinaryHeap;
use test::Bencher;

#[bench]
fn push_seq(b: &mut Bencher) {
    b.iter(|| {
        let n = 1000;
        let mut heap = BinaryHeap::with_capacity(n);
        for i in 0..n {
            heap.push(i);
        }
    });
}

// #[bench]
// fn pop_max_seq(b: &mut Bencher) {
//     b.iter(|| {
//         let n = 1000;
//         let mut heap: BinaryHeap<_> = (0..n).collect();
//         for _ in 0..n {
//             heap.reverse().pop();
//         }
//     });
// }

#[bench]
fn pop_min_seq(b: &mut Bencher) {
    b.iter(|| {
        let n = 1000;
        let mut heap: BinaryHeap<_> = (0..n).collect();
        for _ in 0..n {
            heap.pop();
        }
    });
}
