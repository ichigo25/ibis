#[macro_use]
extern crate serde_derive;
extern crate toml;

mod core;
mod config;


//=============================================================================
// ibisのインタフェース
//
// (1) アプリケーション起動の例
// ```
// use ibis;
//
// fn main()
// {
//      ibis::App::run();
// }
// ```
//=============================================================================
pub struct App {}

impl App
{
    pub fn run()
    {
        crate::core::IbisCore::run();
    }
}

