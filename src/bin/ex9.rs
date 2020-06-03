extern crate ncurses;

use std::process::exit;
use ncurses::*;

fn main() {
    initscr();
    let stdscr = stdscr();

    if !has_colors() {
        endwin();
        println!("Your terminal does not support color");
        exit(1);
    }

    // if !can_change_color() {
    //     endwin();
    //     println!("Your terminal cannot change color");
    //     exit(1);
    // }

    // rgb (0-1000)
    init_color(COLOR_RED, 0, 0, 0);

    start_color();

    init_pair(1, COLOR_RED, COLOR_BLACK);

    attron(COLOR_PAIR(1));
    print_in_middle(stdscr, LINES() / 2, 0, COLS(), "Viola !!! In color ...");
    attroff(COLOR_PAIR(1));

    getch();
    endwin();
}

fn print_in_middle(
    mut win: WINDOW,
    start_y: i32,
    start_x: i32,
    width: i32,
    s: &str,
)
{
    if win.is_null() {
        win = stdscr();
    }
    let mut y = 0;
    let mut x = 0;
    let mut w = width;

    getyx(win, &mut y, &mut x);
    if start_x != 0 {
        x = start_x;
    }
    if start_y != 0 {
        y = start_y;
    }
    if width == 0 {
        w = 80;
    }

    let len = s.len();
    let tmp = (w - len as i32) / 2;
    x += tmp;
    mvwprintw(win, y, x, s);
    refresh();
}
