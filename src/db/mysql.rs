use mysql::*;
use mysql::prelude::*;
use dotenv;

fn conn() -> Pool {
    dotenv::dotenv();
    let url = "mysql://root:password@localhost:3307/db_name";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
}

