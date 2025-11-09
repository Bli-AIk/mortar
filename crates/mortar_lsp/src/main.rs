use std::env::temp_dir;

use mortar_lsp::backend::{
    Backend, detect_system_language, i18n::get_lsp_text, parse_language_from_args,
};
use tokio::io::{stdin, stdout};
use tower_lsp_server::{LspService, Server};
use tracing::{info, subscriber};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    // Determine language from command line args or environment
    let language = parse_language_from_args().unwrap_or_else(detect_system_language);

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

    subscriber::set_global_default(subscriber)
        .expect(&get_lsp_text("unable_set_subscriber", language));

    info!("{}", get_lsp_text("starting_lsp_server", language));
    if parse_language_from_args().is_none() {
        info!(
            "{}: {:?}",
            get_lsp_text("language_auto_detected", language),
            language
        );
    } else {
        info!(
            "{}: {:?}",
            get_lsp_text("language_set_to", language),
            language
        );
    }

    let stdin = stdin();
    let stdout = stdout();

    let (service, socket) = LspService::new(move |client| {
        let backend = Backend::new(client);
        // Set the initial language from command line or environment
        let backend_clone = backend.clone();
        let lang = language;
        tokio::spawn(async move {
            backend_clone.set_language(lang).await;
        });
        backend
    });

    info!("{}", get_lsp_text("lsp_server_ready", language));
    Server::new(stdin, stdout, socket).serve(service).await;
}
