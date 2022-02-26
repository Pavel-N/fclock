use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "fclock",
    about = "Fibonacci clock in a terminal.",
    author = "Pavel Novák"
)]
pub struct FClockArgs {

    // ===== Flags =====

    // --- Centered position ---
    /// Clocks will be centered in terminal, preceeds "width" and "height" option
    #[structopt(short, long)]
    pub centered: bool,

    // --- Borders visible ---
    /// Clocks will be shown with borders
    #[structopt(short, long)]
    pub borders: bool,


    // ===== Options ======

    // --- Size ---
    /// Width of the clock in columns. 
    #[structopt(short = "W", long, default_value = "62")]
    pub width: u16,
    /// Height of the clock in rows. 
    #[structopt(short = "H", long, default_value = "20")]
    pub height: u16,

    // --- Position ---
    /// Position along the x axis in columns. 
    /// Preceeded by the "centered" flag.
    #[structopt(short, long = "x-axis", default_value = "0")]
    pub x: u16,
    /// Position along the y axis in rows.
    /// Poreceeded by the "centered" flag.
    #[structopt(short, long = "y-axis", default_value = "0")]
    pub y: u16

}
