use super::{Context, QueryRequest};

#[derive(Serialize, Deserialize, Debug)]
pub struct Table{
    pub name: String,
    pub table_type: TableType
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TableType{
    #[serde(rename = "BASE TABLE")]
    Table,
    #[serde(rename = "VIEW")]
    View,
    #[serde(rename = "MODEL")]
    Model
}

pub struct CreateTable {
    pub name: String,
    pub integration: String,
    pub query: String,
}

impl CreateTable {
    pub fn new(integration: &str, name: &str, query: &str) -> Self {
        Self {
            name: name.to_owned(),
            integration: integration.to_owned(),
            query: query.to_owned(),
        }
    }
}

impl From<CreateTable> for QueryRequest {
    fn from(value: CreateTable) -> Self {
        let query = format!(
            "CREATE TABLE {}.{} ({})",
            value.integration, value.name, value.query
        );
        let context = Context::new(value.integration);

        Self {
            query,
            context,
        }
    }
}
