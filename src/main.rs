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
        eprintln!("Application error: {}",e);
        process::exit(1);
    }

}
