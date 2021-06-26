use mysql::prelude::*;

use crate::db_core::{CheckOperationError, CheckedOperation, Column, Database, Operation};

pub struct MySql {
    connection: mysql::Conn,
}

impl MySql {
    pub fn new(username: &str, password: &str, hostname: &str, database_name: &str) -> Self {
        let url = format!(
            "mysql://{}:{}@{}/{}",
            username, password, hostname, database_name
        );

        let connection = mysql::Conn::new(url).unwrap();
        MySql { connection }
    }
}

impl Database for MySql {
    type Type = mysql::consts::ColumnType;
    fn check_operation(
        &mut self,
        operation: Operation,
    ) -> Result<CheckedOperation<Self::Type>, CheckOperationError> {
        let stmt = self
            .connection
            .prep(operation.sql.clone())
            .map_err(|error| CheckOperationError::PrepareStatementError(error.to_string()))?;
        let columns = stmt
            .columns()
            .iter()
            .map(|column| Column {
                name: column.name_str().to_string(),
                typ: column.column_type(),
            })
            .collect::<Vec<_>>();
        Ok(CheckedOperation {
            operation,
            columns,
            parameters_count: stmt.num_params() as usize,
        })
    }
}
