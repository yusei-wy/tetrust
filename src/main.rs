mod entity;
use getch_rs::{Getch, Key};
use std::{thread, time};

use entity::*;

// TODO: enum でリストしてコマンドにしたい
// CSI シーケンスによって端末を操作している
fn clear_screen() {
    println!("\x1b[2J\x1b[H\x1b[?25l");
}

fn clear_cursor_pos() {
    print!("\x1b[H");
}

fn show_cursor() {
    println!("\x1b[?25h");
}

fn is_collision(field: &Field, pos: &Position, block: BlockKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if field[y + pos.y][x + pos.x] & BLOCKS[block as usize][y][x] == 1 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let field: Field = [
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let mut pos = Position { x: 4, y: 0 };
    let g = Getch::new();

    clear_screen();

    loop {
        // generate field for render
        let mut field_buf = field;

        // natural fall
        let new_pos = Position {
            x: pos.x,
            y: pos.y + 1,
        };

        if !is_collision(&field, &new_pos, BlockKind::I) {
            // updated pos
            pos = new_pos;
        }

        // set block info in the drawing field
        for y in 0..4 {
            for x in 0..4 {
                if BLOCKS[BlockKind::I as usize][y][x] == 1 {
                    field_buf[y + pos.y][x + pos.x] = 1;
                }
            }
        }

        // render field
        clear_cursor_pos();
        (0..FIELD_HEIGHT).for_each(|y| {
            (0..FIELD_WIDTH).for_each(|x| {
                if field_buf[y][x] == 1 {
                    print!("[]");
                } else {
                    print!(" .");
                }
            });
            println!();
        });

        thread::sleep(time::Duration::from_millis(1000));

        // waiting for key input
        match g.getch() {
            Ok(Key::Left) => {
                let new_pos = Position {
                    x: pos.x - 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, BlockKind::I) {
                    pos = new_pos;
                }
            }
            Ok(Key::Down) => {
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, BlockKind::I) {
                    pos = new_pos;
                }
            }
            Ok(Key::Right) => {
                let new_pos = Position {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, BlockKind::I) {
                    pos = new_pos;
                }
            }
            Ok(Key::Char('q')) => break,
            _ => (),
        }
    }

    show_cursor();
}
