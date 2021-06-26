#[derive(Debug)]
pub struct Operation {
    pub file_path: String,
    pub sql: String,
}

#[derive(Debug)]
pub struct CheckedOperation<Type> {
    pub operation: Operation,
    pub parameters_count: usize,
    pub columns: Vec<Column<Type>>,
}

#[derive(Debug)]
pub struct Column<Type> {
    pub name: String,
    pub typ: Type,
}

#[derive(Debug)]
pub enum CheckOperationError {
    PrepareStatementError(String),
}

pub trait Database {
    type Type;
    fn check_operation(
        &mut self,
        operation: Operation,
    ) -> Result<CheckedOperation<Self::Type>, CheckOperationError>;
}
