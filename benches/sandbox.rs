#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

use std::{hint::black_box, str::FromStr};

use divan::{bench, Bencher};
use fastbuf::{Buf, Buffer};
use serialization::{Decode, Encode};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

#[derive(
    Debug,
    serialization::Serializable,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Clone,
    bitcode::Decode,
    bitcode::Encode,
)]
pub struct Log {
    pub address: Address,
    pub identity: String,
    pub userid: String,
    pub date: String,
    pub request: String,
    pub code: u16,
    pub size: u64,
}

#[derive(
    bitcode::Decode,
    bitcode::Encode,
    Debug,
    serialization::Serializable,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Clone,
)]
pub struct Logs {
    pub logs: Vec<Log>,
}

#[derive(
    Debug,
    serialization::Serializable,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Clone,
    Copy,
    bitcode::Decode,
    bitcode::Encode,
)]
pub struct Address {
    pub x0: u8,
    pub x1: u8,
    pub x2: u8,
    pub x3: u8,
}

fn model() -> Logs {
    Logs {
        logs: vec![
            Log {
                address: Address {
                    x0: 11,
                    x1: 22,
                    x2: 33,
                    x3: 44,
                },
                identity: String::from_str("ABCD").unwrap(),
                userid: String::from_str("ABCD").unwrap(),
                date: String::from_str("ABCD").unwrap(),
                request: String::from_str("ABCD").unwrap(),
                code: 55,
                size: 66,
            };
            10
        ],
    }
}
#[bench(sample_count = 1000, sample_size = 1000)]
fn encode(bencher: Bencher) {
    let mut buf = Buffer::<1000>::new();
    let model = model();
    bencher.bench_local(|| {
        let mut enc = PacketEncoder::new(&mut buf);
        black_box(&model.encode(&mut enc).unwrap());
        unsafe { buf.set_filled_pos(0) };
    });
}

#[bench(sample_count = 1000, sample_size = 1000)]
fn decode(bencher: Bencher) {
    let mut buf = Buffer::<1000>::new();
    let mut enc = PacketEncoder::new(&mut buf);
    let model = model();
    black_box(model.encode(&mut enc)).unwrap();
    bencher.bench_local(|| {
        let mut dec = PacketDecoder::new(&mut buf);
        black_box(&Logs::decode(&mut dec));
        unsafe { buf.set_pos(0) };
    });
}

fn main() {
    divan::main();
}

#[bench(sample_count = 1000, sample_size = 1000)]
fn bitcode_encode(bencher: Bencher) {
    let model = model();
    bencher.bench_local(|| {
        black_box(&bitcode::encode(&model));
    });
}

#[bench(sample_count = 1000, sample_size = 1000)]
fn bitcode_decode(bencher: Bencher) {
    let model = model();
    let bytes = bitcode::encode(&model);
    let bytes = &bytes;
    bencher.bench_local(|| {
        black_box(&bitcode::decode::<Logs>(bytes).unwrap());
    });
}
