//! Ansinator is collection of image convertion to character representation programs.

mod args;
mod ascii;
mod braile;
mod block;
mod uniblock;
mod utils;

use clap::Parser;
use args::AnsinatorArgs;
use std::process;

fn main() {
    let args = AnsinatorArgs::parse();
    //println!("{:?}", args);

    if let Err(e) = 
        match &args.command {
            args::AnsinatorCommands::Ascii(ascii) => {
                ascii.run()
            },
            args::AnsinatorCommands::Braile(braile) => {
                braile.run()
            },
            args::AnsinatorCommands::Block(block) => {
                block.run() 
            },
            args::AnsinatorCommands::Uniblock(uniblock) => {
                uniblock.run() 
            },
        }
    {
        eprintln!("{}",e);
        process::exit(1);
    }

}
