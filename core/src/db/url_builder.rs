use std::env;

pub struct DatabaseUrlBuilder<'a> {
    username_env: Option<&'a str>,
    username_default: &'a str,
    password_env: Option<&'a str>,
    password_default: &'a str,
    database_env: Option<&'a str>,
    database_default: &'a str,
}

impl<'a> DatabaseUrlBuilder<'a> {
    pub fn new() -> Self {
        DatabaseUrlBuilder {
            username_env: Some("PGUSER"),
            username_default: "postgres",
            password_env: Some("PGPASSWORD"),
            password_default: "",
            database_env: Some("PGDATABASE"),
            database_default: "",
            // TODO: port, host
        }
    }

    pub fn with_user_from_env(&mut self, name: &'a str) -> &mut Self {
        self.username_env = Some(name);
        self
    }

    pub fn with_password_from_env(&mut self, name: &'a str) -> &mut Self {
        self.password_env = Some(name);
        self
    }

    pub fn with_database_from_env(&mut self, name: &'a str) -> &mut Self {
        self.database_env = Some(name);
        self
    }

    pub fn build(&self) -> String {
        let user = self.username_env.and_then(|n| env::var(n).ok());
        let password = self.password_env.and_then(|n| env::var(n).ok());
        let database = self.database_env.and_then(|n| env::var(n).ok());

        format!("postgres://{}@localhost/{}?password={}",
                user.as_deref().unwrap_or(self.username_default),
                password.as_deref().unwrap_or(self.password_default),
                database.as_deref().unwrap_or(self.database_default),
        )
    }
}