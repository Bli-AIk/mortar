use std::env::temp_dir;

use mortar_lsp::backend::Backend;
use tokio::io::{stdin, stdout};
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

    info!("Starting the Mortar LSP server...");

    let stdin = stdin();
    let stdout = stdout();

    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}