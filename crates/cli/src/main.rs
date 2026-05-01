use cli::executor::MagoExecutor;
use cli::mago::command::MagoCommand;
use cli::runtime::{resolve_mago_bin, setup_logging};
use mago_core::terms::{EDITOR_WORKSPACE_ROOT_ENV, MAGO_BIN_ENV};
use std::io;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    DidOpenTextDocumentParams, DidSaveTextDocumentParams, InitializeParams, InitializeResult,
    InitializedParams, MessageType, SaveOptions, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncOptions, TextDocumentSyncSaveOptions, Url,
};
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tracing::{debug, error, info, warn};

struct Backend {
    client: Client,
    executor: MagoExecutor,
}

impl Backend {
    async fn analyze_and_publish(&self, uri: Url) {
        let Ok(file_path) = uri.to_file_path() else {
            warn!(uri = %uri, "==== ignoring non-file URI");
            self.client
                .log_message(MessageType::WARNING, format!("Skipping non-file URI: {uri}"))
                .await;
            return;
        };

        let mut diagnostics = Vec::new();

        for command_type in MagoCommand::diagnostics_pipeline() {
            match self.executor.run_check(&file_path, command_type) {
                Ok(mut current_diagnostics) => diagnostics.append(&mut current_diagnostics),
                Err(error_value) => {
                    let message = error_value.to_string();
                    error!(
                        command_type = command_type.as_str(),
                        file_path = %file_path.display(),
                        error = %error_value,
                        "==== mago phase failed"
                    );
                    self.client
                        .log_message(
                            MessageType::ERROR,
                            format!(
                                "Mago {command_type} failed for {file_path}: {message}",
                                command_type = command_type.as_str(),
                                file_path = file_path.display()
                            ),
                        )
                        .await;
                }
            }
        }

        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        let result = InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(false),
                        })),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            },
            ..Default::default()
        };

        Ok(result)
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::INFO, "Mago LSP Wrapper initialized!").await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.analyze_and_publish(params.text_document.uri).await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.analyze_and_publish(params.text_document.uri).await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let _log_guard = setup_logging()?;
    info!("==== starting mago zed wrapper");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let mago_bin = resolve_mago_bin();

    debug!(
        mago_binary = %mago_bin,
        mago_bin_env = MAGO_BIN_ENV,
        workspace_root_env = EDITOR_WORKSPACE_ROOT_ENV,
        "==== wrapper runtime context"
    );

    let (service, socket) =
        LspService::new(|client| Backend { client, executor: MagoExecutor::new(mago_bin.clone()) });
    debug!("==== lsp service created");
    Server::new(stdin, stdout, socket).serve(service).await;
    info!("==== mago zed wrapper stopped");

    Ok(())
}
