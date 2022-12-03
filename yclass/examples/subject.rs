//! This example exists for the purpose of testing the main application.

use std::{iter::repeat_with, thread::park};

#[repr(C)]
struct Foo {
    values: [u16; 10],
    p1: Box<[u32; 10]>,
    p2: Box<[f32; 10]>,
}

fn main() {
    let foo = Foo {
        values: repeat_with(|| fastrand::u16(..255))
            .take(10)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        p1: Box::new(
            repeat_with(|| fastrand::u32(..255))
                .take(10)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        ),
        p2: Box::new(
            repeat_with(fastrand::f32)
                .take(10)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        ),
    };
    println!("Address: {:p}", &foo);

    park();
}
