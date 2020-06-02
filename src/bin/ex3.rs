extern crate ncurses;

use ncurses::*;

fn main() {
    let msg = "Just a string";
    let mut row = 0;
    let mut col = 0;

    initscr();
    let stdscr = stdscr();

    // print message at the center of the screen
    getmaxyx(stdscr, &mut row, &mut col);

    mvprintw(row / 2, (col - msg.len() as i32) / 2, msg);
    mvprintw(
        row - 2,
        0,
        &format!("This screen has {} rows and {} columns\n", row, col),
    );

    addstr(
        "Try resizing your window(if possible) and then run this program again",
    );
    refresh();
    getch();
    endwin();
}
