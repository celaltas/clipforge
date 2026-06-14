use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardContentType {
    Text,
    Image,
    File,
}

impl ToSql for ClipboardContentType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let val = match self {
            ClipboardContentType::Text => "text",
            ClipboardContentType::Image => "image",
            ClipboardContentType::File => "file",
        };
        Ok(ToSqlOutput::from(val))
    }
}

impl FromSql for ClipboardContentType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "text" => Ok(ClipboardContentType::Text),
            "image" => Ok(ClipboardContentType::Image),
            "file" => Ok(ClipboardContentType::File),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClipboardEntry {
    pub id: i64,
    pub content: String,
    pub content_type: ClipboardContentType,
    pub created_at: i64,
    pub pinned: bool,
}
