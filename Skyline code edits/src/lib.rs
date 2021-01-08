#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;
use smash::app::utility::get_kind;
use smash::lib::lua_const::*;

mod mario;
mod falco;
mod custom;
mod captain;
mod mk;
mod wolf;
mod greninja;
mod cloud;
mod bayo;
mod squirtle;
mod ivysaur;
mod zard;
mod diddy;
mod isabelle;
mod joker;
mod marth;
mod sheik;
mod ness;
mod pichu;
mod inkling;
mod roy;
mod fox;
mod chrom;
mod ike;
mod ken;
mod mewtwo;
mod incin;
mod byleth;
mod dk;

#[skyline::main(name = "50xx_mod")]
pub fn main() {
    mario::install();
    falco::install();
    custom::install();
    captain::install();
    mk::install();
    wolf::install();
    greninja::install();
    cloud::install();
    bayo::install();
    squirtle::install();
    ivysaur::install();
    zard::install();
    diddy::install();
    isabelle::install();
    joker::install();
    marth::install();
    sheik::install();
    ness::install();
    pichu::install();
    inkling::install();
    roy::install();
    fox::install();
    chrom::install();
    ike::install();
    ken::install();
    mewtwo::install();
    incin::install();
    dk::install();
}