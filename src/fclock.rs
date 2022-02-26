use crate::opt_args::FClockArgs;
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
    fblock5: FBlock,
    fblock3: FBlock,
    fblock2: FBlock,
    fblock1a: FBlock,
    fblock1b: FBlock
}

impl Default for FClock {
    fn default() -> Self {
        Self { 
            width: 60, 
            height: 20, 
            pos: FClockPos::Absolute(0, 0), 
            fblock5: FBlock::default(), 
            fblock3: FBlock::default(), 
            fblock2: FBlock::default(), 
            fblock1a: FBlock::default(), 
            fblock1b: FBlock::default()
        }
    }
}

impl FClock {
    pub fn from_optional_args(term_size: Rect) -> Self {
        let mut clock = Self::default();

        let args = FClockArgs::from_args();

        // --- Size --- 
        if args.width > term_size.width
           || args.height > term_size.height {
            panic!("Clocks of selected size cant fit in terminal!")
        } else {
            if args.borders && (args.width < 22 || args.height < 13) {
                panic!("Minimal size of clocks with border is width=22 and height=13!");
            } else if args.width < 6 || args.height < 3 {
                panic!("Minimal size of clocks without border is width=6 and height=3!");  // TODO: This funtion should return result and panic in main
            } else {
                clock.width = args.width;
                clock.height = args.height;
            }
        }

        // --- Centered position ---
        if args.centered {
            clock.pos = FClockPos::Centered;
        }

        // --- Position ---  // TODO: Ensure valid position
        else {
            if (args.width + args.x) > term_size.width 
               || (args.height + args.width) > term_size.height {
                panic!("Invalid position entered!")
            } else {
                clock.pos = FClockPos::Absolute(args.x, args.y);
            }
        }

        // --- Borders visible ---
        if args.borders {
            clock.fblock5 = clock.fblock5.with_borders();
            clock.fblock3 = clock.fblock3.with_borders();
            clock.fblock2 = clock.fblock2.with_borders();
            clock.fblock1a = clock.fblock1a.with_borders();
            clock.fblock1b = clock.fblock1b.with_borders();
        }
    
        clock
    }

    pub fn update(&mut self) {
        let time = Local::now();
        let mut hours = time.hour12().1;
        let mut minutes = (time.minute() + 2) / 5;  // Rounded to nearest 5

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
        determine_block(&mut self.fblock5, &mut 5, &mut hours, &mut minutes);

        // Block3
        determine_block(&mut self.fblock3, &mut 3, &mut hours, &mut minutes);

        // Block2
        determine_block(&mut self.fblock2, &mut 2, &mut hours, &mut minutes);
        
        // Block1a
        determine_block(&mut self.fblock1a, &mut 1, &mut hours, &mut minutes);
        
        // Block1b
        determine_block(&mut self.fblock1b, &mut 1, &mut hours, &mut minutes);
    }

    pub fn draw<B: Backend>(&self,  f: &mut Frame<B>) {
        let size = f.size();

        let r: Rect = match self.pos {
            FClockPos::Centered => Rect {
                width: self.width,
                height: self.height,
                x: size.width / 2 - self.width / 2,
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


        self.fblock5.draw(f, layout5[0]);
        self.fblock3.draw(f, layout3[0]);
        self.fblock2.draw(f, layout2[1]);
        self.fblock1a.draw(f, layout1[0]);
        self.fblock1b.draw(f, layout1[1]);
    }
}