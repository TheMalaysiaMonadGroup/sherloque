use crate::ast::ast;

pub struct Environment {
    tables: Vec<Table>,
}

pub struct Table {}

pub enum TypecheckError {}

pub fn typecheck_schema(statements: Vec<ast::Statement>) -> Result<(), TypecheckError> {
    todo!()
}

pub fn typecheck_operation(statement: ast::Statement) -> Result<(), TypecheckError> {
    todo!()
}
