extern crate ncurses;

use ncurses::*;

const WIDTH: i32 = 30;
const HEIGHT: i32 = 10;

static CHOICES: [&str; 5] =
    ["Choice 1", "Choice 2", "Choice 3", "Choice 4", "Exit"];

// the size of char pointer in C is 8 bytes
// same as:
//
// ```
// (std::mem::size_of::<char>() * 2 * CHOICES.len()) /
//     (std::mem::size_of::<char>() * 2);
// ```
static N_CHOICES: usize = CHOICES.len();

fn main() {
    let mut highlight: usize = 1;
    let mut choice = 0;

    initscr();
    clear();
    noecho();

    cbreak();

    let start_x = (80 - WIDTH) / 2;
    let start_y = (24 - HEIGHT) / 2;

    let menu_win = newwin(HEIGHT, WIDTH, start_y, start_x);
    keypad(menu_win, true);
    mvprintw(
        0,
        0,
        "Use arrow keys to go up and down, Press enter to select a choice",
    );
    refresh();
    print_menu(menu_win, highlight);

    loop {
        let c = wgetch(menu_win);
        match c {
            KEY_UP => {
                if highlight == 1 {
                    highlight = N_CHOICES;
                } else {
                    highlight -= 1;
                }
            },
            KEY_DOWN => {
                if highlight == N_CHOICES {
                    highlight = 1;
                } else {
                    highlight += 1;
                }
            },
            10 => {
                // enter
                choice = highlight;
            },
            _ => {
                mvprintw(
                    24,
                    0,
                    &format!(
                        "Character pressed is = {} Hopefully it can be \
                         printed as '{}'",
                        c, c
                    ),
                );
                refresh();
            },
        }
        print_menu(menu_win, highlight);
        if choice != 0 {
            break;
        }
    }
    mvprintw(
        23,
        0,
        &format!(
            "You chose choice {} with choice string {}\n",
            choice,
            CHOICES[choice - 1],
        ),
    );
    clrtoeol();
    refresh();
    endwin();
}

fn print_menu(menu_win: WINDOW, highlight: usize) {
    let x = 2;
    let mut y = 2;

    box_(menu_win, 0, 0);
    for (i, c) in CHOICES.iter().enumerate() {
        if highlight == i + 1 {
            // high light the present choice
            wattron(menu_win, A_REVERSE());
            mvwprintw(menu_win, y, x, &c.to_string());
            wattroff(menu_win, A_REVERSE());
        } else {
            mvwprintw(menu_win, y, x, &c.to_string());
        }
        y += 1;
    }
    wrefresh(menu_win);
}
