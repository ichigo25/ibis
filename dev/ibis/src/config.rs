#![allow(dead_code)]

use std::default::Default;
use std::fs;
use std::io::{BufReader, Read};

use anyhow::Result;


//=============================================================================
// IbisConfig
//=============================================================================
#[derive(Debug)]
pub(crate) struct IbisConfig
{
    pub server_config: IbisServerType,
    pub app_config: IbisAppConfig,
    pub logger_config: IbisLoggerType,
}

impl IbisConfig
{
    //=========================================================================
    // デフォルトのファイル名で設定ファイルを読み込み
    //=========================================================================
    pub(crate) fn init() -> Self
    {
        let default_file = "config/config.toml";
        IbisConfig::init_with_file(default_file)
    }

    //=========================================================================
    // ファイル名を指定して設定ファイルを読み込み
    //=========================================================================
    pub(crate) fn init_with_file(file: &str) -> Self
    {
        // tomlの中身を読み込み
        let toml_content = match Self::read_file(file.to_owned())
        {
            Ok(toml_content) => toml_content,
            Err(e) =>
            {
                // ファイルが見つからなければデフォルト値を使う
                println!("[WARN] can't read a config file: {}", e);
                println!("[INFO] use default config");
                return IbisConfig::default();
            },
        };

        // すべての設定
        let config: toml::Value = match toml::from_str(&toml_content)
        {
            Ok(config) => config,
            Err(e) =>
            {
                // 設定が読み込めなければデフォルト値を使う
                println!("[WARN] can't deserialize config: {}", e);
                println!("[INFO] use default config");
                return IbisConfig::default();
            },
        };

        // server_config
        let server_config = match config.get("server")
        {
            Some(s) =>
            {
                // [server]セクションのkindから使用するサーバのタイプを指定
                let server_type = match s["kind"].as_str()
                {
                    Some(s) => s,
                    None =>
                    {
                        // kindが読み込めなかったらデフォルト値を返す
                        println!("[INFO] use default server config");
                        "tokio"
                    },
                };

                // kindがtokioであれば
                if server_type == "tokio"
                {
                    let tokio_config = match toml::from_str(&config["tokio"].to_string())
                    {
                        Ok(tokio_config) => tokio_config,
                        Err(e) =>
                        {
                            // [tokio]セクションが読み込めなければデフォルト値を使う
                            println!("[WARN] can't deserialize tokio config: {}", e);
                            println!("[INFO] use default tokio config");
                            IbisServerTokioConfig::default()
                        },
                    };

                    IbisServerType::Tokio(tokio_config)
                }
                else
                {
                    // [kind]セクションの値が不正ならばデフォルト値を使う
                    println!("[WARN] invalid server kind ({})", server_type);
                    println!("[INFO] use default server config");
                    IbisServerType::default()
                }
            },
            None =>
            {
                // 設定ファイルに[server]セクションがなければデフォルト値を使う
                println!("[WARN] not found [server] section in {}", file);
                println!("[INFO] use default server config");
                IbisServerType::default()
            }
        };

        // app_config
        let app_config = match config.get("app")
        {
            Some(_) =>
            {
                match toml::from_str(&config["app"].to_string())
                {
                    Ok(s) => s,
                    Err(e) =>
                    {
                        // [app]セクションが読み込めなければデフォルト値を使う
                        println!("[WARN] can't deserialize application config: {}", e);
                        println!("[INFO] use default application config");
                        IbisAppConfig::default()
                    },
                }
            },
            None =>
            {
                // 設定ファイルに[app]セクションがなければデフォルト値を使う
                println!("[WARN] not found [app] section in {}", file);
                println!("[INFO] use default application config");
                IbisAppConfig::default()
            }
        };

        // logger_config
        let logger_config = match config.get("logger")
        {
            Some(s) =>
            {
                // [logger]セクションのkindから使用するロガーのタイプを指定
                let logger_type = match s["kind"].as_str()
                {
                    Some(s) => s,
                    None =>
                    {
                        // kindが読み込めなかったらデフォルト値を返す
                        println!("[INFO] use default logger config");
                        "tracing"
                    },
                };

                // kindがtracingであれば
                if logger_type == "tracing"
                {
                    let tracing_config = match toml::from_str(&config["tracing"].to_string())
                    {
                        Ok(logger_type) => logger_type,
                        Err(e) =>
                        {
                            // [tracing]セクションが読み込めなければデフォルト値を使う
                            println!("[WARN] can't deserialize tracing config: {}", e);
                            println!("[INFO] use default tracing config");
                            IbisLoggerTracingConfig::default()
                        },
                    };

                    IbisLoggerType::Tracing(tracing_config)
                }
                else
                {
                    // [kind]セクションの値が不正ならばデフォルト値を使う
                    println!("[WARN] invalid logger kind ({})", logger_type);
                    println!("[INFO] use default logger config");
                    IbisLoggerType::default()
                }
            },
            None =>
            {
                // 設定ファイルに[logger]セクションがなければデフォルト値を使う
                println!("[WARN] not found [logger] section in {}", file);
                println!("[INFO] use default logger config");
                IbisLoggerType::default()
            }
        };

        Self
        {
            server_config,
            app_config,
            logger_config,
        }
    }

    //=========================================================================
    // ファイルを読み込み
    //=========================================================================
    fn read_file(path_: String) -> Result<String>
    {
        let mut file_content = String::new();

        let mut fr = fs::File::open(path_)
            .map(|f| BufReader::new(f))?;

        fr.read_to_string(&mut file_content)?;

        Ok(file_content)
    }
}

impl IbisConfig
{
    //=========================================================================
    // application名を取得
    //=========================================================================
    pub(crate) fn get_app_name(&self) -> &str
    {
        &self.app_config.app_name
    }

    //=========================================================================
    // application versionを取得
    //=========================================================================
    pub(crate) fn get_app_version(&self) -> &str
    {
        &self.app_config.version
    }

    //=========================================================================
    // サーバのaddressを取得
    //=========================================================================
    pub(crate) fn get_server_address(&self) -> &str
    {
        let address = match &self.server_config
        {
            IbisServerType::Tokio(tokio_config) =>
            {
                &tokio_config.address
            }
        };
        &address
    }

    //=========================================================================
    // サーバのportを取得
    //=========================================================================
    pub(crate) fn get_server_port(&self) -> &str
    {
        let port = match &self.server_config
        {
            IbisServerType::Tokio(tokio_config) =>
            {
                &tokio_config.port
            }
        };
        &port
    }

    //=========================================================================
    // サーバのworker_threadsを取得
    //=========================================================================
    pub(crate) fn get_server_worker_threads(&self) -> usize
    {
        match &self.server_config
        {
            IbisServerType::Tokio(tokio_config) =>
            {
                tokio_config.worker_threads
            }
        }
    }

    //=========================================================================
    // サーバのblocking_threadsを取得
    //=========================================================================
    pub(crate) fn get_server_blocking_threads(&self) -> usize
    {
        match &self.server_config
        {
            IbisServerType::Tokio(tokio_config) =>
            {
                tokio_config.blocking_threads
            }
        }
    }

    //=========================================================================
    // サーバのkeep_aliveを取得
    //=========================================================================
    pub(crate) fn get_server_keep_alive(&self) -> u64
    {
        match &self.server_config
        {
            IbisServerType::Tokio(tokio_config) =>
            {
                tokio_config.keep_alive
            }
        }
    }

    //=========================================================================
    // サーバのstack_sizeを取得
    //=========================================================================
    pub(crate) fn get_server_stack_size(&self) -> usize
    {
        match &self.server_config
        {
            IbisServerType::Tokio(tokio_config) =>
            {
                tokio_config.stack_size
            }
        }
    }

    //=========================================================================
    // ロガーのlog_levelを取得
    //=========================================================================
    pub(crate) fn get_logger_log_level(&self) -> &str
    {
        let log_level = match &self.logger_config
        {
            IbisLoggerType::Tracing(tracing_config) =>
            {
                &tracing_config.log_level
            }
        };
        &log_level
    }

    //=========================================================================
    // ロガーのlogfile_pathを取得
    //=========================================================================
    pub(crate) fn get_logger_logfile_path(&self) -> &str
    {
        let logfile_path = match &self.logger_config
        {
            IbisLoggerType::Tracing(tracing_config) =>
            {
                &tracing_config.logfile_path
            }
        };
        &logfile_path
    }

    //=========================================================================
    // ロガーのlogfile_nameを取得
    //=========================================================================
    pub(crate) fn get_logger_logfile_name(&self) -> &str
    {
        let logfile_name = match &self.logger_config
        {
            IbisLoggerType::Tracing(tracing_config) =>
            {
                &tracing_config.logfile_name
            }
        };
        &logfile_name
    }
}

impl Default for IbisConfig
{
    //=========================================================================
    // 初期値の設定
    //=========================================================================
    fn default() -> Self
    {
        Self
        {
            server_config: IbisServerType::default(),
            app_config: IbisAppConfig::default(),
            logger_config: IbisLoggerType::default(),
        }
    }
}


//=============================================================================
// IbisServerType
//=============================================================================
#[derive(Debug)]
pub(crate) enum IbisServerType
{
    Tokio(IbisServerTokioConfig),
}

impl Default for IbisServerType
{
    //=========================================================================
    // 初期値の設定
    //=========================================================================
    fn default() -> Self
    {
        Self::Tokio(IbisServerTokioConfig::default())
    }
}

//=============================================================================
// IbisServerTokioConfig
//=============================================================================
#[derive(Debug, Deserialize)]
pub(crate) struct IbisServerTokioConfig
{
    pub worker_threads: usize,
    pub blocking_threads: usize,
    pub keep_alive: u64,
    pub stack_size: usize,
    pub address: String,
    pub port: String,
}

impl Default for IbisServerTokioConfig
{
    //=========================================================================
    // 初期値の設定
    //=========================================================================
    fn default() -> Self
    {
        Self
        {
            worker_threads: 5,
            blocking_threads: 50,
            keep_alive: 60,
            stack_size: 3145728,
            address: "127.0.0.1".to_string(),
            port: "8000".to_string(),
        }
    }
}


//=============================================================================
// IbisAppConfig
//=============================================================================
#[derive(Debug, Deserialize)]
pub(crate) struct IbisAppConfig
{
    pub app_name: String,
    pub version: String,
}

impl Default for IbisAppConfig
{
    //=========================================================================
    // 初期値の設定
    //=========================================================================
    fn default() -> Self
    {
        Self
        {
            app_name: "xxx".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}


//=============================================================================
// IbisLoggerType
//=============================================================================
#[derive(Debug)]
pub(crate) enum IbisLoggerType
{
    Tracing(IbisLoggerTracingConfig),
}

impl Default for IbisLoggerType
{
    //=========================================================================
    // 初期値の設定
    //=========================================================================
    fn default() -> Self
    {
        Self::Tracing(IbisLoggerTracingConfig::default())
    }
}

//=============================================================================
// IbisLoggerTracingConfig
//=============================================================================
#[derive(Debug, Deserialize)]
pub(crate) struct IbisLoggerTracingConfig
{
    pub log_level: String,
    pub logfile_path: String,
    pub logfile_name: String,
}

impl Default for IbisLoggerTracingConfig
{
    //=========================================================================
    // 初期値の設定
    //=========================================================================
    fn default() -> Self
    {
        Self
        {
            log_level: "debug".to_string(),
            logfile_path: "./output/logs".to_string(),
            logfile_name: "app_log".to_string(),
        }
    }
}


