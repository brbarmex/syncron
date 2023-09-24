use r2d2_postgres::{PostgresConnectionManager, r2d2};
use tokio_postgres::NoTls;

use crate::{repository, iofile, usecase, daemon};


pub fn start_up() {

    let config = "host=localhost user=postgres".parse().unwrap();
    let tls_connector = NoTls;

    let manager = PostgresConnectionManager::new(
        config,
        tls_connector,
    );
    
    let pool = r2d2::Pool::new(manager).unwrap();
    let repository = Box::<repository::snapshot_repository::Repository>::new(repository::snapshot_repository::new(pool));
    let io_file = Box::<iofile::iofile::IoFile>::new(iofile::iofile::new());
    let backup_service = Box::<usecase::backup::Backup>::new(usecase::backup::Backup::new(io_file, repository));

    daemon::DaemonSet::new(backup_service).run();


}