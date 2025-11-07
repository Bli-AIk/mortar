use std::env::temp_dir;
use std::process;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use mortar_lsp::backend::Backend;
use tokio::io::{stdin, stdout};
use tokio::signal;
use tower_lsp_server::{LspService, Server};
use tracing::{subscriber, info};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    let cache_dir = temp_dir().join("mortar-lsp");
    let file_appender = tracing_appender::rolling::hourly(cache_dir, "mortar-lsp.log");
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_writer(non_blocking_writer)
        .with_ansi(false)
        .finish();

    subscriber::set_global_default(subscriber).expect("Unable to set global default subscriber");

    info!("启动 Mortar LSP 服务器...");

    let stdin = stdin();
    let stdout = stdout();

    // 创建共享的退出标志
    let exit_flag = Arc::new(AtomicBool::new(false));
    let exit_flag_clone = exit_flag.clone();

    let (service, socket) = LspService::new(move |client| {
        let backend = Backend::new(client);
        backend
    });

    // 启动LSP服务器
    let server_task = tokio::spawn(async move {
        let server = Server::new(stdin, stdout, socket);
        server.serve(service).await;
        info!("LSP服务器任务完成");
    });

    // 监听关闭信号
    let signal_task = tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for ctrl-c");
        info!("接收到 Ctrl+C，设置退出标志");
        exit_flag_clone.store(true, Ordering::Relaxed);
    });

    // 等待任务完成或超时
    tokio::select! {
        _ = server_task => {
            info!("LSP服务器正常退出");
        }
        _ = signal_task => {
            info!("接收到关闭信号，优雅关闭LSP服务器...");
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            process::exit(0);
        }
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(3600)) => {
            // 1小时超时保护
            info!("LSP服务器运行时间过长，自动退出");
            process::exit(0);
        }
    }
}