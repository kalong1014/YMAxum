// Copyright © [企业全称信息以便后期替换] 2020-2026 YMAxum Framework. All Rights Reserved.
// 自研原创代码，未经授权禁止复制/修改

use crate::core::route_generator::RouteGenerator;
use crate::core::state::AppState;
use sqlx::{Executor, MySqlPool, PgPool, SqlitePool};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct NewbieScene {
    pub app_state: Option<Arc<AppState>>,
    db_config: HashMap<String, String>,
    generated_apis: Vec<String>,
    mysql_pool: Option<MySqlPool>,
    postgres_pool: Option<PgPool>,
    sqlite_pool: Option<SqlitePool>,
}

impl Default for NewbieScene {
    fn default() -> Self {
        Self::new()
    }
}

impl NewbieScene {
    pub fn new() -> Self {
        Self {
            app_state: None,
            db_config: HashMap::new(),
            generated_apis: Vec::new(),
            mysql_pool: None,
            postgres_pool: None,
            sqlite_pool: None,
        }
    }

    pub fn set_app_state(&mut self, app_state: Arc<AppState>) {
        self.app_state = Some(app_state);
    }

    pub async fn configure_database(
        &mut self,
        db_type: &str,
        conn_str: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.db_config
            .insert("type".to_string(), db_type.to_string());
        self.db_config
            .insert("connection_string".to_string(), conn_str.to_string());

        match db_type {
            "mysql" => self.init_mysql(conn_str).await?,
            "postgres" => self.init_postgres(conn_str).await?,
            "sqlite" => self.init_sqlite(conn_str).await?,
            _ => return Err("Unsupported database type".into()),
        }

        Ok(())
    }

    async fn init_mysql(&mut self, conn_str: &str) -> Result<(), Box<dyn std::error::Error>> {
        let pool = MySqlPool::connect(conn_str).await?;
        self.mysql_pool = Some(pool);

        if let Some(ref pool) = self.mysql_pool {
            pool.execute(
                "CREATE TABLE IF NOT EXISTS users (
                id INT AUTO_INCREMENT PRIMARY KEY,
                username VARCHAR(50) NOT NULL UNIQUE,
                email VARCHAR(100) NOT NULL UNIQUE,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            )
            .await?;

            pool.execute(
                "CREATE TABLE IF NOT EXISTS posts (
                id INT AUTO_INCREMENT PRIMARY KEY,
                title VARCHAR(255) NOT NULL,
                content TEXT,
                user_id INT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES users(id)
            )",
            )
            .await?;
        }

        Ok(())
    }

    async fn init_postgres(&mut self, conn_str: &str) -> Result<(), Box<dyn std::error::Error>> {
        let pool = PgPool::connect(conn_str).await?;
        self.postgres_pool = Some(pool);

        if let Some(ref pool) = self.postgres_pool {
            pool.execute(
                "CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                username VARCHAR(50) NOT NULL UNIQUE,
                email VARCHAR(100) NOT NULL UNIQUE,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            )
            .await?;

            pool.execute(
                "CREATE TABLE IF NOT EXISTS posts (
                id SERIAL PRIMARY KEY,
                title VARCHAR(255) NOT NULL,
                content TEXT,
                user_id INT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES users(id)
            )",
            )
            .await?;
        }

        Ok(())
    }

    async fn init_sqlite(&mut self, conn_str: &str) -> Result<(), Box<dyn std::error::Error>> {
        let pool = SqlitePool::connect(conn_str).await?;
        self.sqlite_pool = Some(pool);

        if let Some(ref pool) = self.sqlite_pool {
            pool.execute(
                "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                email TEXT NOT NULL UNIQUE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            )
            .await?;

            pool.execute(
                "CREATE TABLE IF NOT EXISTS posts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content TEXT,
                user_id INTEGER,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES users(id)
            )",
            )
            .await?;
        }

        Ok(())
    }

    pub fn generate_standard_apis(&mut self) -> Vec<String> {
        let apis = vec![
            "/api/users GET - List all users".to_string(),
            "/api/users POST - Create a new user".to_string(),
            "/api/users/:id GET - Get a user by ID".to_string(),
            "/api/users/:id PUT - Update a user".to_string(),
            "/api/users/:id DELETE - Delete a user".to_string(),
            "/api/posts GET - List all posts".to_string(),
            "/api/posts POST - Create a new post".to_string(),
            "/api/posts/:id GET - Get a post by ID".to_string(),
            "/api/posts/:id PUT - Update a post".to_string(),
            "/api/posts/:id DELETE - Delete a post".to_string(),
        ];

        self.generated_apis = apis.clone();
        apis
    }

    pub fn generate_crud_routes(&self, table_name: &str) -> axum::Router<Arc<AppState>> {
        if let Some(ref app_state) = self.app_state {
            let generator = RouteGenerator::new(app_state.clone());
            generator.generate_crud_routes(table_name)
        } else {
            panic!("AppState not set");
        }
    }

    pub fn get_generated_apis(&self) -> &Vec<String> {
        &self.generated_apis
    }

    pub fn get_db_config(&self) -> &HashMap<String, String> {
        &self.db_config
    }
}

impl crate::scene::SceneAdapter for NewbieScene {
    fn name(&self) -> &'static str {
        "newbie"
    }

    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Newbie scene started");
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Newbie scene stopped");
        Ok(())
    }
}

