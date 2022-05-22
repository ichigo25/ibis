use crate::config;

use std::time::Duration;
use std::file;
use std::str::FromStr;

use tokio::runtime::Builder;
use tokio::net::TcpListener;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };

use tracing::{ Level, info, warn, error };
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::fmt::writer::MakeWriterExt;


//=============================================================================
// IbisCore
//=============================================================================
pub(crate) struct IbisCore {}

impl IbisCore
{
    //=========================================================================
    // アプリケーションのバージョン取得
    //=========================================================================
    pub fn get_version() -> &'static str
    {
        "1.0.0"
    }


    //=========================================================================
    // アプリケーションの起動
    //=========================================================================
    pub fn run()
    {
        println!(r"
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
 /$$$$$$ /$$       /$$          
|_  $$_/| $$      |__/          
  | $$  | $$$$$$$  /$$  /$$$$$$$
  | $$  | $$__  $$| $$ /$$_____/
  | $$  | $$  \ $$| $$|  $$$$$$ 
  | $$  | $$  | $$| $$ \____  $$
 /$$$$$$| $$$$$$$/| $$ /$$$$$$$/
|______/|_______/ |__/|_______/ 

$$ Web Application Framework $$

version: {}
author: Ichigo
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        ", Self::get_version());

        // 設定値の初期化
        let config = config::IbisConfig::init();

        let log_level = match tracing::Level::from_str(config.get_logger_log_level())
        {
            Ok(level) => level,
            Err(e) =>
            {
                warn!("undefined log level: {}", e);
                info!("set log level as INFO");
                Level::TRACE
            },
        };

        //=====================================================================
        // Tracingの設定
        let logfile = tracing_appender::rolling::daily(
            config.get_logger_logfile_path(),
            config.get_logger_logfile_name()
        );
        let stdout = std::io::stdout.with_max_level(log_level);

        let subscriber = FmtSubscriber::builder()
            .with_writer(stdout.and(logfile))
            .with_max_level(log_level)
            .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_file(true)
            .with_line_number(true)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        info!("Start {} (version: {})",
            config.get_app_name(),
            config.get_app_version()
        );

        //=====================================================================
        // Tokioのランタイム
        let runtime = match Builder::new_multi_thread()
            .enable_io()
            .worker_threads(config.get_server_worker_threads())
            .max_blocking_threads(config.get_server_blocking_threads())
            .thread_name(format!("{}-thread", config.get_app_name()))
            .thread_keep_alive(
                Duration::from_secs(config.get_server_keep_alive())
            )
            .thread_stack_size(config.get_server_stack_size())
            .build()
            {
                Ok(runtime) => runtime,
                Err(e) =>
                {
                    error!("runtime error: {}", e);
                    return;
                },
            };

        runtime.block_on(async
        {
            // アドレスとポートをTcpListenerにバインディング
            let listener = match TcpListener::bind(format!(
                "{}:{}",
                config.get_server_address(),
                config.get_server_port()
            )).await
            {
                Ok(listener) => listener,
                Err(e) =>
                {
                    error!("tcp listener error: {}", e);
                    return;
                },
            };
            info!("listening on: {}:{}",
                config.get_server_address(),
                config.get_server_port()
            );

            loop
            {
                // ソケットと接続先情報の取得
                let (mut socket, data) = match listener.accept().await
                {
                    Ok((socket, data)) => (socket, data),
                    Err(e) =>
                    {
                        error!("application error: {}", e);
                        return;
                    },
                };

                info!("accept: {}", data);

                tokio::spawn(async move
                {
                    let mut buf = [0; 1024];

                    loop
                    {
                        // クライアントのリクエスト（ソケット）を書き出し
                        let _n = match socket.read(&mut buf).await
                        {
                            Ok(n) if n == 0 => return,  // ソケットがclose()
                            Ok(n) => n,
                            Err(e) =>
                            {
                                error!("failed to read from socket: {}", e);
                                return;
                            },
                        };

                        let response = "test";

                        // クライアントへのレスポンス
                        if let Err(e) = socket.write_all(
                            response.as_bytes()
                        ).await
                        {
                            error!("failed to write to socket: {}", e);
                            return;
                        }
                    }
                });
            }
        })
    }
}

