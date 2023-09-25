mod bootstrap;
mod daemon;
mod entity;
mod iofile;
mod repository;
mod usecase;

fn main() {
    bootstrap::start_up();
}
