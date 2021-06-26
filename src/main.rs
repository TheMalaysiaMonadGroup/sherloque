// mod ast;
// mod generate;
// mod tests;

// mod typecheck;
use mysql::prelude::*;

fn main() {
    let username = "root";
    let password = "";
    let hostname = "localhost";
    let database_name = "nation";
    let url = format!(
        "mysql://{}:{}@{}/{}",
        username, password, hostname, database_name
    );

    let pool = mysql::Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let query = "
        select 
            continents.name, count(*), regions.name
        from 
            continents inner join regions 
                on continents.continent_id = regions.continent_id 
        where regions.name = ?
        group by continents.name
        ";
    let stmt = conn.prep(query).unwrap();
    let columns = stmt
        .columns()
        .iter()
        .map(|column| (column.name_str().to_string(), column.column_type()))
        .collect::<Vec<(String, mysql::consts::ColumnType)>>();

    println!("query = {}", query);
    println!("columns = {:#?}", columns);
    println!("number of parameters = {:#?}", stmt.num_params());
}
