pub enum Language {
    /// Internal language used for testing
    TypesOnly,
}

pub fn generate(
    schema: String,
    operation_name: String,
    operation: String,
    language: Language,
) -> Result<String, String> {
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;
    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...
    let schema = Parser::parse_sql(&dialect, &schema);
    let operation_name = Parser::parse_sql(&dialect, &operation);

    todo!()
}
