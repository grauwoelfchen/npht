extern crate ncurses;

use ncurses::*;

fn main() {
    initscr();
    let stdscr = stdscr();

    cbreak();

    keypad(stdscr, true);

    let height = 3;
    let width = 10;
    let mut start_y = (LINES() - height) / 2;
    let mut start_x = (COLS() - width) / 2;

    waddstr(stdscr, "Press F1 to exit");
    refresh();

    let mut my_win = create_newwin(height, width, start_y, start_x);

    loop {
        match getch() {
            KEY_F1 => break,
            KEY_LEFT => {
                destroy_win(my_win);
                start_x -= 1;
                my_win = create_newwin(height, width, start_y, start_x);
            },
            KEY_RIGHT => {
                destroy_win(my_win);
                start_x += 1;
                my_win = create_newwin(height, width, start_y, start_x);
            },
            KEY_UP => {
                destroy_win(my_win);
                start_y -= 1;
                my_win = create_newwin(height, width, start_y, start_x);
            },
            KEY_DOWN => {
                destroy_win(my_win);
                start_y += 1;
                my_win = create_newwin(height, width, start_y, start_x);
            },
            _ => {},
        }
    }

    endwin();
}

fn create_newwin(
    height: i32,
    width: i32,
    start_y: i32,
    start_x: i32,
) -> WINDOW
{
    let local_win = newwin(height, width, start_y, start_x);
    // print a border around the window
    box_(local_win, 0, 0);
    wrefresh(local_win);
    local_win
}

fn destroy_win(local_win: WINDOW) {
    // box_(local_win, ' ' as u32, ' ' as u32);
    wborder(
        local_win, ' ' as u32, ' ' as u32, ' ' as u32, ' ' as u32, ' ' as u32,
        ' ' as u32, ' ' as u32, ' ' as u32,
    );
    wrefresh(local_win);
    delwin(local_win);
}
