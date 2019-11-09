extern crate ncurses;

use ncurses::*;
use std::sync::mpsc::Receiver;
use crate::types::GuiMsg;


const INSTRUCTIONS: &str = "Press F1 to exit. Press 'g' to goto. Press 'm' to make a message.\nPress 'c' to clear. Press 'r' to resize the C U B E.";

pub fn launch(_gui_rx: Receiver<GuiMsg>)
{
    /* Setup ncurses. */
    initscr();
    raw();

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();

    /* Invisible cursor. */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    /* Status/help info. */
    mvprintw(0, 0, "Use the arrow keys to move");
    put_pos(0, 0);
    //mvprintw(LINES() - 2, 0, &INSTRUCTIONS);
    refresh();

    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);


    let mut window_height: i32 = 3;
    let mut window_width: i32 = 4;

    /* Start in the center. */
    let mut start_y = (max_y - window_height) / 2;
    let mut start_x = (max_x - window_width) / 2;
    let mut win = create_win(start_y, start_x, window_width, window_height);

    let mut ch = getch();
    while ch != KEY_F(1)
    {
        match ch
        {
            KEY_LEFT =>
            {
                start_x -= 1;
                destroy_win(win);
                win = create_win(start_y, start_x, window_width, window_height);
            },
            KEY_RIGHT =>
            {
                start_x += 1;
                destroy_win(win);
                win = create_win(start_y, start_x, window_width, window_height);
            },
            KEY_UP =>
            {
                start_y -= 1;
                destroy_win(win);
                win = create_win(start_y, start_x, window_width, window_height);
            },
            KEY_DOWN =>
            {
                start_y += 1;
                destroy_win(win);
                win = create_win(start_y, start_x, window_width, window_height);
            },
            99 => { // 'c' -> Clear
                clear();
            },
            103 => { // 'g' -> Move window
                mv(1, 0);
                clrtoeol();
                mv(2, 0);
                clrtoeol();
                addstr("Enter x:");
                let mut x = String::new();

                ch = getch();
                while ch != 10 {
                    x.push(ch as u8 as char);
                    addstr(&(ch as u8 as char).to_string());
                    ch = getch();
                }
                match x.parse::<i32>() {
                    Ok(n) => start_x = n,
                    Err(_e) => {
                        start_x = start_x;
                        addstr("Invalid position.");
                    },
                }

                addstr(" | Enter y:");
                let mut y = String::new();
                ch = getch();
                while ch != 10 {
                    y.push(ch as u8 as char);
                    addstr(&(ch as u8 as char).to_string());
                    ch = getch();
                }
                match y.parse::<i32>() {
                    Ok(n) => start_y = n,
                    Err(_e) => {
                        start_y = start_y;
                        addstr("Invalid position.");
                    },
                }
                mv(1, 0);
                clrtoeol();

                mv(2, 0);
                clrtoeol();

            },
            109 => { // Display alert
                mv(1,0);
                clrtoeol();
                mv(2,0);
                clrtoeol();
                mv(1,0);
                addstr("Enter alert message: ");
                let mut s = String::new();
                ch = getch();
                while ch != 10 {
                    s.push(ch as u8 as char);
                    addstr(&(ch as u8 as char).to_string());
                    ch = getch();
                }

                // DIMENSION CODE
                mv(2, 0);
                clrtoeol();
                addstr("Enter x dimension:");
                let mut x = String::new();
                ch = getch();
                while ch != 10 {
                    x.push(ch as u8 as char);
                    addstr(&(ch as u8 as char).to_string());
                    ch = getch();
                }
                let x_i32;
                match x.parse::<i32>() {
                    Ok(n) => x_i32 = n,
                    Err(_e) => {
                        x_i32 = 0;
                        mv(3,0);
                        addstr("Invalid dimension entered.");
                        mv(4,0);

                    },
                }

                addstr(" | Enter y dimension:");
                let mut y = String::new();
                ch = getch();
                while ch != 10 {
                    y.push(ch as u8 as char);
                    addstr(&(ch as u8 as char).to_string());
                    ch = getch();
                }
                let y_i32;
                match y.parse::<i32>() {
                    Ok(n) => y_i32 = n,
                    Err(_e) => {
                        y_i32 = 0;
                        addstr("Invalid dimension entered.");
                    },
                }

                //POSITION CODE
                mv(3, 0);
                clrtoeol();
                addstr("Enter x position:");
                let mut x = String::new();
                ch = getch();
                while ch != 10 {
                    x.push(ch as u8 as char);
                    addstr(&(ch as u8 as char).to_string());
                    ch = getch();
                }
                let x_i32_pos;
                match x.parse::<i32>() {
                    Ok(n) => x_i32_pos = n,
                    Err(_e) => {
                        x_i32_pos = 0;
                        mv(3,0);
                        addstr("Invalid dimension entered.");
                        mv(4,0);

                    },
                }

                addstr(" | Enter y position:");
                let mut y = String::new();
                ch = getch();
                while ch != 10 {
                    y.push(ch as u8 as char);
                    addstr(&(ch as u8 as char).to_string());
                    ch = getch();
                }
                let y_i32_pos;
                match y.parse::<i32>() {
                    Ok(n) => y_i32_pos = n,
                    Err(_e) => {
                        y_i32_pos = 0;
                        addstr("Invalid dimension entered.");
                    },
                } 

                put_alert(x_i32_pos, y_i32_pos, x_i32, y_i32, &s);

                mv(1,0);
                clrtoeol();
                mv(2,0);
                clrtoeol();
                mv(3,0);
                clrtoeol();
                mv(4,0);
                clrtoeol();

            },
            114 => { // Resize main window
                mv(1, 0);
                clrtoeol();
                mv(2, 0);
                clrtoeol();
                addstr("Enter x:");
                let mut x = String::new();

                ch = getch();
                while ch != 10 {
                    x.push(ch as u8 as char);
                    addstr(&(ch as u8 as char).to_string());
                    ch = getch();
                }
                match x.parse::<i32>() {
                    Ok(n) => window_width = n,
                    Err(_e) => {
                        window_width = window_width;
                        addstr("Invalid position.");
                    },
                }

                addstr(" | Enter y:");
                let mut y = String::new();
                ch = getch();
                while ch != 10 {
                    y.push(ch as u8 as char);
                    addstr(&(ch as u8 as char).to_string());
                    ch = getch();
                }
                match y.parse::<i32>() {
                    Ok(n) => window_height = n,
                    Err(_e) => {
                        window_height = window_height;
                        addstr("Invalid position.");
                    },
                }
                mv(1, 0);
                clrtoeol();
                mv(2, 0);
                clrtoeol();
            },
            _ => { }
        }



        mvprintw(0, 0, "Use the arrow keys to move");
        put_pos(start_x, start_y);
        ch = getch();

        if start_x == 0 { start_x = max_x-2; }
        if start_x == max_x-1 { start_x = 1; }
        if start_y == 0 { start_y = max_y-2; }
        if start_y == max_y-1 { start_y = 1; }

        if start_x == 1 && start_y == 1 {
            put_alert(-1, -1, 30, 10, "The quick brown fox jumps over the lazy dog. and actually, I believe you'll find that it's pronounced whomstved... What is ligma? How did I get this disease? What are my options?");
        }
    }

    endwin();
}

fn create_win(start_y: i32, start_x: i32, window_width: i32, window_height: i32) -> WINDOW
{
    let win = newwin(window_height, window_width, start_y, start_x);
    box_(win, 0, 0);
    wrefresh(win);
    win
}

fn destroy_win(win: WINDOW)
{
    let ch = ' ' as chtype;
    wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    wrefresh(win);
    delwin(win);
}

fn put_pos(start_y: i32, start_x: i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    /* Get the screen bounds. */
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    mv(LINES() - 4, 0);
    for _i in 0..max_x {
        addch('-' as u32);
    }
    mv(LINES() - 3, 0);
    clrtoeol();
    attron(A_BOLD());
    mvprintw(LINES() - 3, 0, format!("X: {} Y: {}", start_y, start_x).as_str());
    attroff(A_BOLD());
    mv(LINES() - 2, 0);
    clrtoeol();
    mv(LINES() - 1, 0);
    clrtoeol();
    mvprintw(LINES() - 2, 0, &INSTRUCTIONS);
}

fn put_alert(x_loc: i32, y_loc: i32, x_dim: i32, y_dim: i32, message: &str) {

    let mut max_x = 0;
    let mut max_y = 0;
    let start_x;
    let start_y;

    match x_loc+y_loc {
        -2 => {
            /* Get the screen bounds. */
            getmaxyx(stdscr(), &mut max_y, &mut max_x);
            start_y = max_y / 2;
            start_x = max_x / 2;
        },
        _ => {
            max_x = x_loc;
            max_y = y_loc;
            start_y = max_y;
            start_x = max_x;
        },
    }

    let win = newwin((y_dim)+2, (x_dim)+2, start_y, start_x);
    if message.len() > (x_dim as usize)
    {
        let real_x_dim = x_dim as usize;
        for i in 0..message.len(){
            let i_i32 = i as i32;
            if i == 0 {
                mvprintw(start_y+1+i_i32, start_x+1, &message[0..real_x_dim]);
            } else if real_x_dim*(i+1) >= message.len() {
                mvprintw(start_y+1+i_i32, start_x+1, &message[real_x_dim*(i)..]);
                break;
            } else {
                mvprintw(start_y+1+i_i32, start_x+1, &message[real_x_dim*(i)..real_x_dim*(i+1)]);
            }
        }
    } else {
        mvprintw(start_y+1, start_x+1, &message);
    }
    box_(win, 0, 0);
    wrefresh(win);
}

