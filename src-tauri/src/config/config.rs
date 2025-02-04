use std::fmt::Display;
use std::str::FromStr;

use anyhow::Context;
use semver::Version;
use serde::Deserialize;
use sqlx::sqlite::SqliteConnectOptions;

use infra::env_util::get_env_var;

#[derive(Clone, Debug)]
pub struct Config {
    pub app_env: Environment,
    pub access_control: AccessControlSetting,
    pub db_settings: DatabaseSetting,
    pub application: ApplicationSetting,
    pub websocket: WebsocketSetting,
    pub admin_frontend_path_prefix: String,
}

#[derive(serde::Deserialize, Clone, Debug)]

pub struct AccessControlSetting {
    pub is_enabled: bool,
    pub enable_workspace_access_control: bool,
    pub enable_collab_access_control: bool,
    pub enable_realtime_access_control: bool,
}

// We are using 127.0.0.1 as our host in address, we are instructing our
// application to only accept connections coming from the same machine. However,
// request from the hose machine which is not seen as local by our Docker image.
//
// Using 0.0.0.0 as host to instruct our application to accept connections from
// any network interface. So using 127.0.0.1 for our local development and set
// it to 0.0.0.0 in our Docker images.
//
#[derive(Clone, Debug)]
pub struct ApplicationSetting {
    pub port: u16,
    pub host: String,
}

#[derive(Clone, Debug)]
pub struct DatabaseSetting {
    pub conn_opts: SqliteConnectOptions,
    pub require_ssl: bool,
    pub max_connections: u32,
}

impl Display for DatabaseSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let masked_conn_opts = self.conn_opts.clone();
        write!(
            f,
            "DatabaseSetting {{ conn_opts: {:?}, require_ssl: {}, max_connections: {} }}",
            masked_conn_opts, self.require_ssl, self.max_connections
        )
    }
}

impl DatabaseSetting {
    pub fn connect_options(&self) -> SqliteConnectOptions {
        self.conn_opts.clone()
    }
}

// Default values favor local development.
pub fn get_configuration() -> Result<Config, anyhow::Error> {
    let config = Config {
        app_env: get_env_var("APPFLOWY_ENVIRONMENT", "local")
            .parse()
            .context("fail to get APPFLOWY_ENVIRONMENT")?,
        access_control: AccessControlSetting {
            is_enabled: get_env_var("APPFLOWY_ACCESS_CONTROL", "false")
                .parse()
                .context("fail to get APPFLOWY_ACCESS_CONTROL")?,
            enable_workspace_access_control: get_env_var(
                "APPFLOWY_ACCESS_CONTROL_WORKSPACE",
                "true",
            )
            .parse()
            .context("fail to get APPFLOWY_ACCESS_CONTROL_WORKSPACE")?,
            enable_collab_access_control: get_env_var("APPFLOWY_ACCESS_CONTROL_COLLAB", "true")
                .parse()
                .context("fail to get APPFLOWY_ACCESS_CONTROL_COLLAB")?,
            enable_realtime_access_control: get_env_var("APPFLOWY_ACCESS_CONTROL_REALTIME", "true")
                .parse()
                .context("fail to get APPFLOWY_ACCESS_CONTROL_REALTIME")?,
        },
        db_settings: DatabaseSetting {
            conn_opts: SqliteConnectOptions::from_str(&get_env_var(
                "APPFLOWY_DATABASE_URL",
                "postgres://postgres:password@localhost:5432/postgres",
            ))?,
            require_ssl: get_env_var("APPFLOWY_DATABASE_REQUIRE_SSL", "false")
                .parse()
                .context("fail to get APPFLOWY_DATABASE_REQUIRE_SSL")?,
            max_connections: get_env_var("APPFLOWY_DATABASE_MAX_CONNECTIONS", "40")
                .parse()
                .context("fail to get APPFLOWY_DATABASE_MAX_CONNECTIONS")?,
        },
        application: ApplicationSetting {
            port: get_env_var("APPFLOWY_APPLICATION_PORT", "8000").parse()?,
            host: get_env_var("APPFLOWY_APPLICATION_HOST", "0.0.0.0"),
        },
        websocket: WebsocketSetting {
            heartbeat_interval: get_env_var("APPFLOWY_WEBSOCKET_HEARTBEAT_INTERVAL", "6")
                .parse()?,
            client_timeout: get_env_var("APPFLOWY_WEBSOCKET_CLIENT_TIMEOUT", "60").parse()?,
            min_client_version: get_env_var("APPFLOWY_WEBSOCKET_CLIENT_MIN_VERSION", "0.5.0")
                .parse()?,
        },
        admin_frontend_path_prefix: get_env_var("APPFLOWY_ADMIN_FRONTEND_PATH_PREFIX", ""),
    };
    Ok(config)
}

/// The possible runtime environment for our application.
#[derive(Clone, Debug, Deserialize)]
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl FromStr for Environment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => anyhow::bail!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WebsocketSetting {
    pub heartbeat_interval: u8,
    pub client_timeout: u8,
    pub min_client_version: Version,
}
