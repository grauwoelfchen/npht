extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();
    let stdscr = stdscr();

    start_color();

    init_pair(1, COLOR_CYAN, COLOR_BLACK);
    waddstr(stdscr, "A Big string which i didn't care to type fully ");

    // y, x, n, attr, color
    mvchgat(0, 0, -1, A_BLINK(), 1);

    refresh();
    getch();
    endwin();
}
