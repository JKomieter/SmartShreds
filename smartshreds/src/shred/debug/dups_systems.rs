use std::io::{self, Write};

use termion::{
    self, color, event::{Event, Key}, input::{MouseTerminal, TermRead}, raw::IntoRawMode, style
};

use crate::shred::{dir_search::DuplicateFile, error::SmartShredsError};

#[derive(Debug)]
pub struct ShredTerminal {
    pub cur_pos: Coordinates,
    pub terminal_size: Coordinates,
    pub files: Vec<DuplicateFile>,
}
#[derive(Debug)]
struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl ShredTerminal {
    pub fn init(files: Vec<DuplicateFile>) -> Result<Self, SmartShredsError> {
        let terminal_size = Self::get_terminal_size()?;
        let cur_pos = Coordinates { x: 1, y: 1 };
        Ok(ShredTerminal {
            cur_pos,
            terminal_size,
            files
       })
    }

    fn get_terminal_size() -> Result<Coordinates, SmartShredsError> {
        let (x, y) = termion::terminal_size()?;
        Ok(Coordinates { x, y })
    }

    pub fn run(&mut self) -> Result<(), SmartShredsError> {
        let stdin = io::stdin();
        let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode()?.lock());
        
        for key in stdin.events() {
            let evt = key?;
            match evt {
                Event::Key(Key::Char('q')) => break,
                Event::Key(Key::Up) => {
                    self.inc_y();
                    self.initiate_display()?;
                }
                _ => {}
            }
            stdout.flush()?;
        }

        Ok(())
    }

    pub fn initiate_display(&mut self) -> Result<(), SmartShredsError> {
        let pos = &self.cur_pos;
        let (old_x, old_y) = (pos.x, pos.y);
        println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        println!(
            "{}{}Duplicate files found in the directory:{}\n",
            style::Bold,
            color::Fg(color::Yellow),
            style::Reset
        );

        for (index, file) in self.files.iter().enumerate() {
            println!(
                "{}{}{}) - {}{}",
                style::Bold,
                color::Fg(color::Red),
                index + 1,
                file,
                style::Reset
            );
        }
        self.set_pos(old_x, old_y); 
        Ok(())
    }

    fn set_pos(&mut self, x: u16, y: u16) {
        self.cur_pos.x = x;
        self.cur_pos.y = y;
        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
        );
    }

    // fn inc_x(&mut self) {
    //     if self.cur_pos.x < self.terminal_size.x {
    //         self.cur_pos.x += 1;
    //     }
    //     println!(
    //         "{}",
    //         termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
    //     );
    // }

    // fn dec_x(&mut self) {
    //     if self.cur_pos.x > 1 {
    //         self.cur_pos.x -= 1;
    //     }
    //     println!(
    //         "{}",
    //         termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
    //     );
    // }

    fn inc_y(&mut self) {
        self.cur_pos.y += 1;
        println!("Going up");
        println!(
            "{}",
            termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
        );
    }

    // fn dec_y(&mut self) {
    //     if self.cur_pos.y > 1 {
    //         self.cur_pos.y -= 1;
    //     }
    //     println!(
    //         "{}",
    //         termion::cursor::Goto(self.cur_pos.x as u16, self.cur_pos.y as u16)
    //     );
    // }
}
