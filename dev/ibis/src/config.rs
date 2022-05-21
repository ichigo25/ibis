#![allow(dead_code)]

use std::default::Default;
use std::fs;
use std::io::{BufReader, Read};

use anyhow::Result;
use tracing::{ info, warn };


//=============================================================================
// IbisConfig
//=============================================================================
#[derive(Debug)]
pub(crate) struct IbisConfig
{
    pub server_config: IbisServerType,
    pub app_config: IbisAppConfig,
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
                warn!("can't read a config file: {}", e);
                info!("use default config");
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
                warn!("can't deserialize config: {}", e);
                info!("use default config");
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
                        info!("use default application config");
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
                            // [server]セクションが読み込めなければデフォルト値を使う
                            warn!("can't deserialize application config: {}", e);
                            info!("use default application config");
                            IbisServerTokioConfig::default()
                        },
                    };

                    IbisServerType::Tokio(tokio_config)
                }
                else
                {
                    // [server]セクションの値が不正ならばデフォルト値を使う
                    warn!("invalid server kind ({})", server_type);
                    info!("use default application config");
                    IbisServerType::default()
                }
            },
            None =>
            {
                // 設定ファイルに[server]セクションがなければデフォルト値を使う
                warn!("not found [app] section in {}", file);
                info!("use default application config");
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
                        warn!("can't deserialize application config: {}", e);
                        info!("use default application config");
                        IbisAppConfig::default()
                    },
                }
            },
            None =>
            {
                // 設定ファイルに[app]セクションがなければデフォルト値を使う
                warn!("not found [app] section in {}", file);
                info!("use default application config");
                IbisAppConfig::default()
            }
        };

        Self
        {
            server_config,
            app_config,
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

