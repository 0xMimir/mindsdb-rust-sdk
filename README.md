# mindsdb
The MindsDB SDK library provides an easy-to-use API for interacting with the [MindsDB](https://github.com/mindsdb/mindsdb) machine learning platform. With this library, you can quickly build and deploy predictive models powered by state-of-the-art machine learning algorithms.

## Getting started

You can add library using cargo or download it from git
```
use mindsdb_sdk::MindsDB;

let mindsdb = MindsDB::new("http://localhost:47335");
```

## Currently covered features

* CRUD actions related to databases
* Read actions related to projects
* Creation and listing of tables

## In progress features
* CRUD actions related to models
* CRUD actions related to views

## Error handling

MindsDB errors are handled with `Error` enum, it contains error for libraries used, `Error::InternalError` is error returned when query is invalid, but syntax is accepted by api, some of examples are:
```
// Invalid db name
InternalError("Can't connect to db: 'invalid_db_name'")
// When invalid server or auth options are invalid
InternalError("Can't connect to db: 'testdb'")
```

