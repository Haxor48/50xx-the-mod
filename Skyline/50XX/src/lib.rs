#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![feature(str_strip)]

pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;
use smash::app::utility::get_kind;
use smash::lib::lua_const::*;


mod ike;
mod ivysaur;
mod ken;
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
mod mewtwo;
mod incin;
mod dk;
mod sephiroth;
mod ganon;
mod corrin;
mod zss;
mod snake;
mod bjr;
mod pika;
mod pit;
mod mac;
mod krool;
mod palutena;
mod lucario;
mod wiifit;
mod pyra;
mod mythra;
mod plant;

#[skyline::main(name = "50xx_mod")]
pub fn main() {
    ivysaur::installIvysaur();
    ken::installKen();
    mario::installMario();
    falco::installFalco();
    custom::installCustom();
    captain::installCaptain();
    mk::installMk();
    wolf::installWolf();
    greninja::installGreninja();
    cloud::installCloud();
    bayo::installBayo();
    squirtle::installSquirtle();
    zard::installZard();
    diddy::installDiddy();
    isabelle::installIsabelle();
    joker::installJoker();
    marth::installMarth();
    sheik::installSheik();
    ness::installNess();
    pichu::installPichu();
    inkling::installInkling();
    roy::installRoy();
    fox::installFox();
    chrom::installChrom();
    ike::installIke();
    mewtwo::installMewtwo();
    incin::installIncin();
    dk::installDk();
    sephiroth::installSephiroth();
    ganon::installGanon();
    corrin::installCorrin();
    zss::installZss();
    snake::installSnake();
    bjr::installBjr();
    pika::installPika();
    pit::installPit();
    mac::installMac();
    krool::installKRool();
    palutena::installPalutena();
    lucario::installLucario();
    wiifit::installWiifit();
    pyra::installPyra();
    mythra::installMythra();
    plant::installPlant();
}