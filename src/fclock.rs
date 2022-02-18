use crate::OptionalArgs;
use structopt::StructOpt;  // For the from_args() funtion
use chrono::{Local, Timelike};
use tui::{backend::Backend, layout::{Layout, Constraint, Rect}, Frame};

use crate::fblock::{FBlock, FBlockColor};


enum FClockPos {
    Centered,
    Absolute(u16, u16)
}

pub struct FClock {
    width: u16,
    height: u16,
    pos: FClockPos,

    // Fibonacci color blocks
    block5: FBlock,
    block3: FBlock,
    block2: FBlock,
    block1a: FBlock,
    block1b: FBlock
}

impl Default for FClock {
    fn default() -> Self {
        Self { 
            width: 60, 
            height: 20, 
            pos: FClockPos::Absolute(0, 0), 
            block5: FBlock::default(), 
            block3: FBlock::default(), 
            block2: FBlock::default(), 
            block1a: FBlock::default(), 
            block1b: FBlock::default()
        }
    }
}

impl FClock {
    pub fn from_optional_args() -> Self {
        let mut clock = Self::default();

        let opt = OptionalArgs::from_args();
        if opt.centered {
            clock.pos = FClockPos::Centered;
        } else {
            if let Some(size) = opt.size {
                if size.len() == 2 {
                    clock.width = size[0];
                    clock.height = size[1];
                } else {
                    println!("Invalid size argument.");
                    // TODO Print help maybe?
                    return clock;
                }
            }
            
            if let Some(pos) = opt.pos {
                if pos.len() == 2 {
                    clock.pos = FClockPos::Absolute(pos[0], pos[1])
                } else {
                    println!("Invalid position argument.");
                    // TODO Print help maybe?
                    return clock;
                }
            }
        }

        if opt.borders {
            clock.block5 = clock.block5.with_borders();
            clock.block3 = clock.block3.with_borders();
            clock.block2 = clock.block2.with_borders();
            clock.block1a = clock.block1a.with_borders();
            clock.block1b = clock.block1b.with_borders();
        }
    
        clock
    }

    pub fn update(&mut self) {
        let time = Local::now();
        let mut hours = time.hour12().1;
        let mut minutes = (time.minute() as f32 / 5.0) as u32;

        // Hours = Red
        // Minutes = Green
        // Both = Blue

        fn determine_block(block: &mut FBlock, block_val: &mut u32, hours: &mut u32,  minutes: &mut u32) {
            if hours >= block_val && minutes < block_val {
                block.color = FBlockColor::Hours;
                *hours -= *block_val;
            } else if hours >= block_val && minutes >= block_val {
                block.color = FBlockColor::Both;
                *hours -= *block_val;
                *minutes -= *block_val;
            } else if hours < block_val && minutes >= block_val {
                block.color = FBlockColor::Minutes;
                *minutes -= *block_val;
            } else {
                block.color = FBlockColor::None;
            }
        }

        // Block5
        determine_block(&mut self.block5, &mut 5, &mut hours, &mut minutes);

        // Block3
        determine_block(&mut self.block3, &mut 3, &mut hours, &mut minutes);

        // Block2
        determine_block(&mut self.block2, &mut 2, &mut hours, &mut minutes);
        
        // Block1a
        determine_block(&mut self.block1a, &mut 1, &mut hours, &mut minutes);
        
        // Block1b
        determine_block(&mut self.block1b, &mut 1, &mut hours, &mut minutes);
    }

    pub fn draw<B: Backend>(&self,  f: &mut Frame<B>) {
        let size = f.size();

        let r: Rect = match self.pos {
            FClockPos::Centered => Rect {
                width: self.width,
                height: self.height,
                x: size.width / 2 - self.width / 2,  // TODO: Add check for terminal size
                y: size.height / 2 - self.height / 2 
            },
            FClockPos::Absolute(x, y) => Rect {
                width: self.width,
                height: self.height,
                x,
                y 
            }
        };
        let layout5 = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .constraints([Constraint::Ratio(5, 8), Constraint::Ratio(2, 8)].as_ref())
            .split(r);
            
        let layout3 = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints([Constraint::Ratio(3, 5), Constraint::Ratio(2, 5)].as_ref())
            .split(layout5[1]);
            
        let layout2 = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
            .split(layout3[1]);

        let layout1 = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(layout2[0]);


        self.block5.draw(f, layout5[0]);
        self.block3.draw(f, layout3[0]);
        self.block2.draw(f, layout2[1]);
        self.block1a.draw(f, layout1[0]);
        self.block1b.draw(f, layout1[1]);
    }
}