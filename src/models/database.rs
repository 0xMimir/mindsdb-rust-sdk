use crate::{Error, Result};

use super::{Context, QueryRequest};

#[derive(Debug, Deserialize, Serialize)]
pub struct Database {
    pub name: String,
    pub db_type: DbType,
    pub engine: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum DbType {
    System,
    Project,
    Data,
}

pub struct CreateDatabaseRequestParams {
    pub name: Option<String>,
    pub engine: String,

    pub params: CreateDatabaseParams,
}

#[derive(Serialize)]
pub struct CreateDatabaseParams {
    pub host: String,
    pub port: i64,
    pub database: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl CreateDatabaseRequestParams {
    pub fn new(engine: &str, host: &str, port: i64, database: &str) -> Self {
        Self {
            name: None,
            engine: engine.to_owned(),
            params: CreateDatabaseParams {
                host: host.to_owned(),
                port,
                database: database.to_owned(),
                user: None,
                password: None,
            },
        }
    }
    pub fn name(&mut self, name: &str) {
        self.name = Some(name.to_owned())
    }

    pub fn user(&mut self, user: &str) {
        self.params.user = Some(user.to_owned())
    }

    pub fn password(&mut self, password: &str) {
        self.params.password = Some(password.to_owned())
    }
}

impl TryFrom<CreateDatabaseRequestParams> for QueryRequest {
    type Error = Error;
    fn try_from(value: CreateDatabaseRequestParams) -> Result<Self> {
        let name = value.name.unwrap_or_else(|| "display_name".to_owned());
        let parameters = serde_json::to_string(&value.params)?;

        let query = format!(
            "CREATE DATABASE {} WITH ENGINE = {}, PARAMETERS = {}",
            name, value.engine, parameters
        );

        Ok(Self {
            query,
            context: Context {
                db: "mindsdb".to_owned(),
            },
        })
    }
}
