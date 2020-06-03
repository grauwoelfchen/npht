extern crate ncurses;

use ncurses::*;

fn main() {
    let msg = "Enter a string: ";
    let mut row = 0;
    let mut col = 0;

    initscr();
    let stdscr = stdscr();

    // print message at the center of the screen
    getmaxyx(stdscr, &mut row, &mut col);
    mvprintw(row / 2, (col - msg.len() as i32) / 2, msg);

    let mut s = "".to_string();
    getstr(&mut s);

    let res = format!("You entered: {}", s);
    mvprintw(LINES() - 2, (col - res.len() as i32) / 2, &res);
    getch();
    endwin();
}
