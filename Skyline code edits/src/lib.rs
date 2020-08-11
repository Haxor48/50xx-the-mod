#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod mario;
mod falco;
mod custom;
mod captain;
mod mk;
mod wolf;
mod greninja;
mod joker;
mod roy;
mod marth;
mod sheik;

#[skyline::main(name = "acmd_test")]
pub fn main() {
    mario::install();
    falco::install();
    custom::install();
    captain::install();
    mk::install();
    wolf::install();
    greninja::install();
    joker::install();
    roy::install();
    sheik::install();
    marth::install();
}