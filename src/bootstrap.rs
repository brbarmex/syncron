use std::env;

use r2d2_postgres::{r2d2, PostgresConnectionManager};
use tokio_postgres::NoTls;

use crate::{daemon, iofile, repository, usecase};

pub fn start_up() {
    let str_conn = dotenv::var("PG_DB_STR_CONN").unwrap();
    let config = str_conn.parse().unwrap();
    let tls_connector = NoTls;

    let manager = PostgresConnectionManager::new(config, tls_connector);

    let pool = r2d2::Pool::new(manager).unwrap();
    let repository = Box::<repository::snapshot_repository::Repository>::new(
        repository::snapshot_repository::new(pool),
    );
    let io_file = Box::<iofile::iofile::IoFile>::new(iofile::iofile::new());
    let backup_service =
        Box::<usecase::backup::Backup>::new(usecase::backup::Backup::new(io_file, repository));

    daemon::DaemonSet::new(backup_service).run();
}
