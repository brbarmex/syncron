use std::env;

mod bootstrap;
mod daemon;
mod entity;
mod iofile;
mod repository;
mod usecase;

fn main() {
    dotenv::dotenv().ok();
    bootstrap::start_up();
}
