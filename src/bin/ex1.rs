extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();
    let stdscr = stdscr();

    waddstr(stdscr, "Hello world !!!");

    refresh();
    getch();
    endwin();
}
