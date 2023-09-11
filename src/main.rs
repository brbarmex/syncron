mod entity;
mod service;
mod cron;
mod config;

fn main()  {
    cron::execute();
}
