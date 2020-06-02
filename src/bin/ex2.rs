extern crate ncurses;

use std::char;

use ncurses::*;

fn main() {
    initscr();
    let stdscr = stdscr();

    raw();
    keypad(stdscr, true);
    noecho();

    addstr("Type any character to see it in bold\n");
    let ch = getch();
    if ch == KEY_F1 {
        addstr("F1 key pressed");
    } else {
        addstr("The pressed key is ");
        attron(A_BOLD() | A_BLINK());

        addstr(
            &format!("{}", char::from_u32(ch as u32).expect("invalid char")));
        attroff(A_BOLD() | A_BLINK());
    }

    refresh();
    getch();
    endwin();
}
