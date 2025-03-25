use welds_connections::Syntax;
use crate::writers::ColumnWriter;

pub(crate) struct SelectColumn {
    pub(crate) col_name: String,
    pub(crate) field_name: String,
    pub(crate) kind: SelectKind,
}

pub(crate) enum SelectKind {
    Column,
    All,
}

impl SelectColumn {
    pub(crate) fn write(&self, syntax: Syntax, alias: &str) -> String {
        match self.kind {
            SelectKind::Column => {
                let writer = ColumnWriter::new(syntax);
                let colname = writer.excape(&self.col_name);
                let fieldname = writer.excape(&self.field_name);

                if colname == fieldname {
                    format!("{}.{}", alias, colname)
                } else {
                    format!("{}.{} AS {}", alias, colname, fieldname)
                }
            }
            SelectKind::All => {
                format!("{}.*", alias)
            }
        }
    }
}
