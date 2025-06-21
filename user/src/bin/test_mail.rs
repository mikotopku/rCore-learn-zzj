#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use user_lib::{fork, getpid, mailread, mailread_available, mailwrite, mailwrite_available, yield_, MAIL_MAXLEN};
use alloc::string::{String, ToString};

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let mut buf =[0u8; MAIL_MAXLEN];
    let bye = "/bye";
    let child = fork();
    let child = child as usize;
    if child == 0 {
        println!("im child {}", getpid());
        while !mailread_available() { yield_(); }
        let mut rdlen = mailread(&mut buf);
        assert!(rdlen > 0);
        let rdstr = str::from_utf8(&buf[0..rdlen as usize]).unwrap();
        println!("[c] {}", rdstr);
        let fpid = rdstr.parse::<usize>().unwrap();
        let helloc = getpid().to_string();
        assert!(mailwrite(fpid, helloc.as_bytes()) > 0);
        println!("[c] sent hello to f");
        loop {
            while !mailread_available() { yield_(); }
            rdlen = mailread(&mut buf);
            let rdstr = str::from_utf8(&buf[0..rdlen as usize]).unwrap();
            assert!(rdlen > 0);
            println!("[c] {}", rdstr);
            if rdstr == bye {
                assert!(mailwrite(fpid, bye.as_bytes()) > 0);
                println!("[c] sent bye to f");
                break;
            }
        }
    }
    else {
        println!("im father {}", getpid());
        let hellof = getpid().to_string();
        assert!(mailwrite(child, hellof.as_bytes()) > 0);
        println!("[f] sent {} to c", hellof);
        while !mailread_available() { yield_(); }
        let rdlen = mailread(&mut buf);
        assert!(rdlen > 0);
        let rdstr = str::from_utf8(&buf[0..rdlen as usize]).unwrap();
        println!("[f] {}", rdstr);
        let cpid = rdstr.parse::<usize>().unwrap();
        assert!(cpid == child);
        for i in 0..32 {
            buf[i] = b'A' + i as u8;
            while !mailwrite_available(cpid) { yield_(); }
            assert!(mailwrite(cpid, &buf[..i + 1]) > 0);
            println!("[f] sent {} to c", i);
        }
        while !mailwrite_available(cpid) { yield_(); }
        assert!(mailwrite(cpid, bye.as_bytes()) > 0);
        println!("[f] sent bye to c");
        while !mailread_available() { yield_(); }
        let rdlen = mailread(&mut buf);
        let rdstr = str::from_utf8(&buf[0..rdlen as usize]).unwrap();
        println!("[f] {}", rdstr);
        if rdstr == bye {
            println!("MAIL TEST PASSED");
        } else {
            println!("SOMETHING WENT WRONG");
        }
    }
    0
}
