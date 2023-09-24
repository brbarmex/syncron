use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread, time::Duration};

pub trait DaemonJob {
    fn execute(&self);
}

pub struct DaemonSet {
    job: Box<dyn DaemonJob>,
}

impl DaemonSet {

    pub fn new(job: Box<dyn DaemonJob>) -> Self {
        DaemonSet { job }
    }

    pub fn run(&self) {

        let running = Arc::new(AtomicBool::new(true));
        let running_clone = Arc::clone(&running);
    
        thread::spawn(move || {
            ctrlc::set_handler(move || {
                println!("Ctrl+C signal received. Shutting down gracefully...");
                running_clone.store(false, Ordering::SeqCst);
            })
            .expect("Error setting Ctrl+C handler");
        });
    
        while running.load(Ordering::SeqCst) {
            self.job.execute();
            thread::sleep(Duration::from_secs(15));
        }
    
        println!("Shutting down...");

    }

}
