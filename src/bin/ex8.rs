extern crate ncurses;

use ncurses::*;

struct Border {
    ls: chtype,
    rs: chtype,
    ts: chtype,
    bs: chtype,
    tl: chtype,
    tr: chtype,
    bl: chtype,
    br: chtype,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            ls: 0,
            rs: 0,
            ts: 0,
            bs: 0,
            tl: 0,
            tr: 0,
            bl: 0,
            br: 0,
        }
    }
}

struct Win {
    start_y: i32,
    start_x: i32,
    height: i32,
    width: i32,
    border: Border,
}

impl Default for Win {
    fn default() -> Self {
        Self {
            start_y: 0,
            start_x: 0,
            height: 0,
            width: 0,
            border: Border::default(),
        }
    }
}

fn main() {
    initscr();
    let stdscr = stdscr();

    start_color();
    cbreak();

    keypad(stdscr, true);
    noecho();

    let mut win = Win::default();

    init_pair(1, COLOR_CYAN, COLOR_BLACK);

    init_win_params(&mut win);
    print_win_params(&win);

    attron(COLOR_PAIR(1));
    waddstr(stdscr, "Press F1 to exit");
    refresh();
    attroff(COLOR_PAIR(1));

    create_box(&win, true);

    loop {
        match getch() {
            KEY_F1 => break,
            KEY_LEFT => {
                create_box(&win, false);
                win.start_x -= 1;
                create_box(&win, true);
            },
            KEY_RIGHT => {
                create_box(&win, false);
                win.start_x += 1;
                create_box(&win, true);
            },
            KEY_UP => {
                create_box(&win, false);
                win.start_y -= 1;
                create_box(&win, true);
            },
            KEY_DOWN => {
                create_box(&win, false);
                win.start_y += 1;
                create_box(&win, true);
            },
            _ => {},
        }
        print_win_params(&win);
    }

    endwin();
}

fn init_win_params(win: &mut Win) {
    win.height = 3;
    win.width = 10;
    win.start_y = (LINES() - win.height) / 2;
    win.start_x = (COLS() - win.width) / 2;
    win.border.ls = '|' as u32;
    win.border.rs = '|' as u32;
    win.border.ts = '-' as u32;
    win.border.bs = '-' as u32;
    win.border.tl = '+' as u32;
    win.border.tr = '+' as u32;
    win.border.bl = '+' as u32;
    win.border.br = '+' as u32;
}

fn print_win_params(win: &Win) {
    let output = format!(
        "window: {} {} {} {}\n",
        win.start_x, win.start_y, win.width, win.height,
    );
    mvprintw(25, 0, &output);
    refresh();
}

fn create_box(win: &Win, flag: bool) {
    let y = win.start_y;
    let x = win.start_x;
    let h = win.height;
    let w = win.width;

    if flag {
        mvaddch(y, x, win.border.tl);
        mvaddch(y, x + w, win.border.tr);
        mvaddch(y + h, x, win.border.bl);
        mvaddch(y + h, x + w, win.border.br);
        mvhline(y, x + 1, win.border.ts, w - 1);
        mvhline(y + h, x + 1, win.border.bs, w - 1);
        mvvline(y + 1, x, win.border.ls, h - 1);
        mvvline(y + 1, x + w, win.border.rs, h - 1);
    } else {
        // clear
        let mut j = y;
        loop {
            if j > (y + h) {
                break;
            }
            let mut i = x;
            loop {
                if i > (x + w) {
                    break;
                }
                mvaddch(j, i, ' ' as u32);
                i += 1;
            }
            j += 1;
        }
    }
}
