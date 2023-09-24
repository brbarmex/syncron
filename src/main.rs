mod entity;
mod usecase;
mod daemon;
mod repository;
mod bootstrap;
mod iofile;

fn main()  {
    
    bootstrap::start_up();
}
