use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread, time::Duration};

use crate::{service, config};

fn scheduled_function() {

    let config = config::EnvConfig::load().unwrap();
    let file_io = service::file::File::new();
    let db_io = service::storage::Postgres::new(&config.db_str_connection);
    let path = String::from(&config.artefact_path);
    let file_name = String::from(&config.file_name);
    let sv = service::snapshot::perform_backup(&file_io, &db_io, path, &file_name);

    match sv {
        Ok(_) => { println!("ok")},
        Err(err) => { eprint!("{}",err)},
    }

    println!("Scheduled function executed.");
}

pub fn execute() {
    
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
        scheduled_function();
        thread::sleep(Duration::from_secs(15));
    }

    println!("Shutting down...");
}
