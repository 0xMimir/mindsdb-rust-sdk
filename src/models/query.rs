#[derive(Deserialize, Serialize, Debug)]
pub struct QueryResponse<T> {
    pub column_names: Vec<String>,
    pub context: Context,
    pub data: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Context {
    pub db: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryRequest {
    pub query: String,
    pub context: Context,
}

impl QueryRequest {
    pub fn new(query: &str, db: &str) -> Self {
        Self {
            query: query.to_owned(),
            context: Context::new(db.to_owned()),
        }
    }

    pub fn new_default(query: &str) -> Self {
        Self {
            query: query.to_owned(),
            context: Context::default(),
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            db: "mindsdb".to_owned(),
        }
    }
}

impl Context {
    pub fn new(db: String) -> Self {
        Self { db }
    }
}
