extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();
    let stdscr = stdscr();

    cbreak();

    keypad(stdscr, true);

    let height = 3;
    let width = 10;
    let start_y = (LINES() - height) / 2;
    let start_x =(COLS() - width) / 2;

    waddstr(stdscr, "Press F1 to exit");
    refresh();

    let mut my_win = newwin(height, width, start_y, start_x);

    loop {
        match getch() {
            KEY_F1 => break,
            KEY_LEFT => {
                delwin(my_win);
                my_win = newwin(height, width, start_y, start_x - 1);
            },
            KEY_RIGHT => {
                delwin(my_win);
                my_win = newwin(height, width, start_y, start_x + 1);
            },
            KEY_UP => {
                delwin(my_win);
                my_win = newwin(height, width, start_y - 1, start_x);
            },
            KEY_DOWN => {
                delwin(my_win);
                my_win = newwin(height, width, start_y + 1, start_x);
            }
            _ => {}
        }
    }

    endwin();
}
