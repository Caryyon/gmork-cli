/* Lets maket his a fast easy to use, easy to add to cli tool
 *
 * 1. lets get the basics down for args, flags and shit like that
 * 2. lets get some colors because that is important to me
 * 3. lets try and really stay true to testing the features
 *
 *
 * !!! this cli tool should be able to run other cli tools like (homebrew, node, stow)
 * !!! it should also be able to run it's own plugins, which can be installed by itself
 *
 *
 *
 */
use clap::Parser;

// lets try and keep the main features needed to run in core_commands
mod core_commands;
use core_commands::init::init;

/// gmork-cli is a general plugin based customizable tool
/// small consumable commands that can be easily tweaked per need
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
// this should likely be adjusted to handle the import of all commands
struct Args {
   /// Usually the first command you run to setup gmork on your machine
   #[clap(short, long, value_parser)]
   init: bool,

}

fn main() {
   let args = Args::parse();
   match args {
       Args { init: true} => {
           init().unwrap()
       },
       _ => {
           println!("I don't know that command, are you sure that's right?");
       }
   }
}
