mod entity;
mod service;

fn main()  {

    let file_io = service::file::File::new();
    let db_io = service::storage::Postgres::new();
    let path = "/Users/wakuba/github/syncron/src/object_plan.txt".to_string();
    let file_name = String::from("object_plan.txt");
    let sv = service::snapshot::perform_backup(&file_io, &db_io, path, &file_name);

    match sv {
        Ok(_) => { println!("ok")},
        Err(err) => { eprint!("{}",err)},
    }

}
