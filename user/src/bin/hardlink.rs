#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{close, fs::Stat, fstat, linkat, open, read, unlinkat, write, OpenFlags};

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let test_str = "Hello, world!";
    let filea = "filea\0";
    let fd = open(filea, OpenFlags::CREATE | OpenFlags::WRONLY);
    assert!(fd > 0);
    let fd = fd as usize;
    write(fd, test_str.as_bytes());
    close(fd);

    let mut stat = Stat::empty();
    let fileb = "fileb\0";
    linkat(filea, fileb);
    let filec = "filec\0";
    linkat(filea, filec);
    let fda = open(filea, OpenFlags::RDONLY) as i32;
    let fdb = open(fileb, OpenFlags::RDONLY) as i32;
    let fdc = open(filec, OpenFlags::RDONLY) as i32;
    assert!(fda > 0);
    assert!(fdb > 0);
    assert!(fdc > 0);
    let mut buf = [0u8; 100];
    let rdl = read(fdb as usize, &mut buf) as usize;
    println!("{}", core::str::from_utf8(&buf[..rdl]).unwrap());
    fstat(fdc, &mut stat);
    println!("{:#?}", stat);
    close(fdb as usize);
    close(fdc as usize);
    unlinkat(fileb);
    fstat(fda, &mut stat);
    println!("{:#?}", stat);
    unlinkat(filec);
    fstat(fda, &mut stat);
    println!("{:#?}", stat);
    0
}
