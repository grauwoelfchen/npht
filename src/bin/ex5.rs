#![feature(stmt_expr_attributes)]

extern crate ncurses;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::BufRead;
use std::process;

use ncurses::*;

fn main() {
    let mut prev: char = ' ';

    let mut row = 0;
    let mut col = 0;

    let mut y = 0;
    let mut x = 0;

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <a rust file name>", args[0]);
        process::exit(1);
    }

    let f = File::open(&args[1]).expect("invalid input");
    let mut r = BufReader::new(f);

    initscr();
    let stdscr = stdscr();
    // print message at the center of the screen
    getmaxyx(stdscr, &mut row, &mut col);

    let mut buf = Vec::<u8>::new();
    while r.read_until(b'\n', &mut buf).expect("read_until") != 0 {
        let s = String::from_utf8(buf).expect("from_utf8");
        for c in s.chars() {
            getyx(stdscr, &mut y, &mut x);
            // at the end of screen
            if y == (row - 1) {
                addstr("<- Press any key ->");
                getch();
                clear();
                mv(0, 0);
            }

            #[rustfmt::skip]
            if prev == '/' && c == '*' { /* this is a comment should be bold */
                attron(A_BOLD());
                getyx(stdscr, &mut y, &mut x);
                mv(y, x - 1);
                addstr(&format!("/{}", c));
            } else {
                addstr(&format!("{}", c));
            }
            refresh();
            if prev == '*' && c == '/' {
                attroff(A_BOLD());
            }
            prev = c;
        }

        // return the ownership
        buf = s.into_bytes();
        buf.clear();
    }

    getch();
    endwin();
}
