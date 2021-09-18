use sqlparser::dialect::Dialect;

/// Support `$` (dollar sign) as variable declaration in sql file.
#[derive(Debug)]
pub struct ScopedDialect {}

impl Dialect for ScopedDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '$'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        (ch >= 'a' && ch <= 'z')
            || (ch >= 'A' && ch <= 'Z')
            || (ch >= '0' && ch <= '9')
            || (ch == '@')
            || ch == '_'
    }
}
