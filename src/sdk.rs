use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{
    models::{CreateDatabaseRequestParams, Database, QueryRequest, QueryResponse, Response},
    Error, Result,
};

pub struct MindsDb {
    server: String,
    client: Client,
}

impl MindsDb {
    /// Returns all databases
    pub async fn get_all_databases(&self) -> Result<Vec<Database>> {
        let params = QueryRequest::new_default("SHOW FULL DATABASES");
        self.query::<QueryResponse<Database>>(params)
            .await
            .map(|response| response.data)
    }

    /// Returns database matching name
    pub async fn get_database(&self, name: &str) -> Result<Database> {
        let databases = self.get_all_databases().await?;
        match databases
            .into_iter()
            .filter(|db| db.name == name)
            .collect::<Vec<_>>()
            .pop()
        {
            Some(db) => Ok(db),
            None => Err(Error::NotFound),
        }
    }

    /// Creates database
    pub async fn create_database(
        &self,
        engine: &str,
        host: &str,
        port: i64,
        database: &str,
        name: Option<&str>,
        user: Option<&str>,
        password: Option<&str>,
    ) -> Result<()> {
        let mut query = CreateDatabaseRequestParams::new(engine, host, port, database);
        if let Some(name) = name {
            query.name(name)
        }
        if let Some(user) = user {
            query.user(user)
        }
        if let Some(password) = password {
            query.password(password)
        }

        let response = self.query::<Response>(query.try_into()?).await?;
        match response.error_message {
            Some(message) => Err(Error::InternalError(message)),
            None => Ok(()),
        }
    }

    /// Deletes database matching name
    pub async fn delete_database(&self, name: &str) -> Result<()> {
        let query = QueryRequest::new_default(&format!("DROP DATABASE {}", name));
        let response = self.query::<Response>(query).await?;
        match response.error_message {
            Some(message) => Err(Error::InternalError(message)),
            None => Ok(()),
        }
    }

    pub async fn run_query<T>(&self, query: &str, db: &str) -> Result<T>
    where
        T: DeserializeOwned + 'static,
    {
        let query = QueryRequest::new(query, db);
        self.query(query).await
    }

    async fn query<T>(&self, params: QueryRequest) -> Result<T>
    where
        T: DeserializeOwned + 'static,
    {
        let url = format!("{}/api/sql/query", self.server);

        self.client
            .post(url)
            .json(&params)
            .send()
            .await?
            .json()
            .await
            .map_err(Error::from)
    }
}
