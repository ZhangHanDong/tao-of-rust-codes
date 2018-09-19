#![feature(test)]
extern crate test;
use test::Bencher;
use std::path::PathBuf;
use csv_challenge::{
    Opt,
    {load_csv, write_csv},
    replace_column,
};
#[bench]
fn bench_read_100times(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(100);
        (0..n).fold(0, |_,_|{test_load_csv();0})
    });
}
fn test_load_csv(){
    let filename = PathBuf::from("./input/challenge.csv");
    load_csv(filename);
}
#[bench]
fn bench_rw_100times(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(100);
        (0..n).fold(0, |_,_|{test_read_write_csv();0})
    });
}

fn test_read_write_csv(){
    let filename = PathBuf::from("./input/challenge.csv");
    let csv_data = load_csv(filename).unwrap();
    let modified_data = replace_column(csv_data, "City", "Beijing").unwrap();
    write_csv(&modified_data, "output/test.csv");
}
