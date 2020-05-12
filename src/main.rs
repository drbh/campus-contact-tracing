//! Actix web juniper example
//!
//! A simple example integrating juniper in actix-web
// use redis_streams::StreamReadReply;
use std::io;
use std::sync::Arc;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

// use chrono::{offset::Utc, DateTime, NaiveDate};
use rustorm::{
    // DbError, FromDao, 
    Pool, 
    // TableName, ToColumnNames, ToDao, ToTableName
};

mod schema;

use crate::schema::{create_schema, Schema};

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    {
        let db_url = "sqlite://rbct.db";

        let mut pool = Pool::new();
        let mut em = pool.em(db_url).unwrap();

        let create_sql = "CREATE TABLE human(
                id integer PRIMARY KEY AUTOINCREMENT,
                name text,
                identifier text
        )";
        let _table_creation_response = em.db().execute_sql_with_return(create_sql, &[]);

        let create_sql = "CREATE TABLE resource(
                id integer PRIMARY KEY AUTOINCREMENT,
                name text,
                location text
        )";
        let _table_creation_response = em.db().execute_sql_with_return(create_sql, &[]);

        let create_sql = "CREATE TABLE interaction(
                id integer PRIMARY KEY AUTOINCREMENT,
                resource_id text,
                human_id text,
                timestamp integer
        )";
        let _table_creation_response = em.db().execute_sql_with_return(create_sql, &[]);

        let create_sql = "CREATE TABLE infection(
                id integer PRIMARY KEY AUTOINCREMENT,
                human_id text,
                timestamp integer
        )";
        let _table_creation_response = em.db().execute_sql_with_return(create_sql, &[]);

    }

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
