use crate::config;

use std::time::Duration;
use std::file;

use tokio::runtime::Builder;
use tokio::net::TcpListener;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };

use tracing::{ Level, info, error };
use tracing_subscriber::FmtSubscriber;


//=============================================================================
// IbisCore
//=============================================================================
pub(crate) struct IbisCore {}

impl IbisCore
{
    //=========================================================================
    // アプリケーションの起動
    //=========================================================================
    pub(crate) fn run()
    {
        // Tracingの設定
        let subscriber = FmtSubscriber::builder()
            .pretty()
            .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc3339())
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_max_level(Level::TRACE)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        // 設定値の初期化
        let config = config::IbisConfig::init();

        info!("Initializing Ibis (version: {})", "1.0.0");
        info!("Start {} (version: {})",
            config.get_app_name(),
            config.get_app_version()
        );

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

