use clap::Parser;
use std::ffi::OsString;

#[derive(Parser)]
#[command(name="rtree")]
#[command(version="1.0")]
#[command(about="Rusty version of tree unix tool", long_about=None)]
pub struct Args{
    #[arg(default_value = ".")]
    pub name: Option<OsString>,
    #[arg(short, long, default_value_t = 2)]
    pub depth: usize,
    #[arg(short, long,)]
    pub all: bool,
}