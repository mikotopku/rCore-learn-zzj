#![no_std]
#![no_main]
#![allow(static_mut_refs)]

use alloc::vec::Vec;
use user_lib::{eventfd, exit, gettid, mutex_create, mutex_lock, mutex_unlock, semaphore_create, semaphore_down, semaphore_up, thread_create, waittid, EFDFlags, read, write};

#[macro_use]
extern crate user_lib;
extern crate alloc;

static mut SCORE: Vec<i32> = Vec::new();
static mut PLAYS: usize = 0;
static mut PLAYERS: usize = 0;
static mut RESBUF: ResultBuf = ResultBuf::zero_init();

#[derive(Clone, Copy, Debug)]
enum Res {
    P1,
    P2,
    DRAW,
}

const BUFMAX: usize = 16;
struct ResultBuf {
    buf: [(usize, usize, Res); BUFMAX],
    head: usize,
    tail: usize,
    len: usize,
    sem_rd: usize,
    sem_wr: usize,
    sem_mut: usize,
    unfinished: usize,
}

impl ResultBuf {
    pub const fn zero_init() -> Self {
        Self {
            buf: [(0, 0, Res::DRAW); BUFMAX],
            head: 0,
            tail: 0,
            len: 0,
            sem_rd: 0,
            sem_wr: 0,
            sem_mut: 0,
            unfinished: 0, 
        }
    }
    pub fn new(writer: usize) -> Self {
        Self {
            buf: [(0, 0, Res::DRAW); BUFMAX],
            head: 0,
            tail: 0,
            len: 0,
            sem_rd: eventfd(0, EFDFlags::EFD_SEMAPHORE) as usize,
            sem_wr: eventfd(BUFMAX, EFDFlags::EFD_SEMAPHORE) as usize,
            sem_mut: eventfd(1, EFDFlags::EFD_SEMAPHORE) as usize,
            unfinished: writer,
        }
    }
    pub fn push(&mut self, res: (usize, usize, Res)) {
        let mut s: [_; 0] = [];
        read(self.sem_wr, &mut s);
        read(self.sem_mut, &mut s);
        self.len += 1;
        self.buf[self.tail % BUFMAX] = res;
        self.tail += 1;
        write(self.sem_mut, &s);
        write(self.sem_rd, &s);
    }
    pub fn pop(&mut self) -> Option<(usize, usize, Res)> {
        if self.len == 0 && self.is_finished() { 
            println!("{} return None", gettid());
            return None;
        }
        let mut s: [_; 0] = [];
        read(self.sem_rd, &mut s);
        read(self.sem_mut, &mut s);
        self.len -= 1;
        self.head += 1;
        let res = self.buf[(self.head - 1) % BUFMAX];
        write(self.sem_mut, &s);
        write(self.sem_wr, &s);
        Some(res)
    }
    pub fn write_last(&mut self, res: (usize, usize, Res)) {
        let mut s: [_; 0] = [];
        read(self.sem_wr, &mut s);
        read(self.sem_mut, &mut s);
        self.len += 1;
        self.buf[self.tail % BUFMAX] = res;
        self.tail += 1;
        self.unfinished -= 1;
        println!("last");
        write(self.sem_mut, &s);
        write(self.sem_rd, &s);
    }
    pub fn is_finished(&self) -> bool {
        let res;
        let mut s: [_; 0] = [];
        read(self.sem_mut, &mut s);
        res = self.unfinished == 0;
        write(self.sem_mut, &s);
        res
    }
}

pub fn j(arg: usize) {
    println!("j");
    let mut n = arg;
    for i in 0..unsafe{PLAYS} {
        n = n * n % 10007;
        let p1 = n % unsafe{PLAYERS};
        n = n * n % 10007;
        let p2 = unsafe{(1 + n % (PLAYERS - 1) + p1) % PLAYERS};
        n = n * n % 10007;
        let w = if n % 3 == 0 { Res::P1 } else if n % 3 == 1 { Res::P2 } else {Res::DRAW};
        let res = if p1 > p2 { (p2, p1, w) } else { (p1, p2, w) };
        unsafe {
            if i == PLAYS - 1 {
                RESBUF.write_last(res);
            } else {
                RESBUF.push(res);
            }
        }
    }
    exit(0);
}



pub fn u() {
    println!("u");
    loop {
        unsafe {
            let res = RESBUF.pop();
            if res.is_none() { exit(0); }
            let res = res.unwrap();
            mutex_lock(res.0);
            mutex_lock(res.1);
            let change = 
            match res.2 {
                Res::P1 => {
                    if SCORE[res.0] >= SCORE[res.1] { (20, -20) }
                    else { (30, -30) }
                },
                Res::P2 => {
                    if SCORE[res.0] <= SCORE[res.1] { (-20, 20) }
                    else { (-30, 30) }
                },
                Res::DRAW => {
                    if SCORE[res.0] < SCORE[res.1] { (10, -10) }
                    else if SCORE[res.0] > SCORE[res.1] { (-10, 10) }
                    else { (0, 0) }
                }
            };
            print!("{:?} ", (SCORE[res.0], SCORE[res.1]));
            SCORE[res.0] += change.0;
            SCORE[res.1] += change.1;
            println!("{:?} {:?}", res, change);
            mutex_unlock(res.1);
            mutex_unlock(res.0);
        }
    }
}

#[unsafe(no_mangle)]
pub fn main(argc: usize, argv: &[&str]) -> i32 {
    assert!(argc == 5);
    let players: usize = argv[1].parse().unwrap();
    let judge: usize = argv[2].parse().unwrap();
    let updator: usize = argv[3].parse().unwrap();
    let plays: usize = argv[4].parse().unwrap();
    unsafe { PLAYS = plays; }
    unsafe { PLAYERS = players; }
    for i in 0..players {
        assert!(mutex_create() == i as isize);
        unsafe { SCORE.push(1000); }
    }
    unsafe {RESBUF = ResultBuf::new(judge); }
    let mut threads = Vec::<usize>::new();
    for i in 0..judge {
        threads.push(thread_create(j as usize, i + 2) as usize);
    }
    for _ in 0..updator {
        threads.push(thread_create(u as usize, 0) as usize);
    }
    while threads.len() != 0 {
        let tid = threads.pop().unwrap();
        if waittid(tid) < 0 { threads.push(tid); }
    }
    for i in 0..players {
        print!("{} ", unsafe {SCORE[i]});
    }
    0
}
