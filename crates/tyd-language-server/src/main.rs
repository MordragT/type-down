use backend::Backend;
use tower_lsp::{LspService, Server};
use tyd_eval::prelude::*;

pub mod backend;
// pub mod kind;
pub mod semantic;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let mut scope = Scope::empty();

    scope
        .register::<BuiltinPlugin>()
        .with("title", "Default title")
        .with("author", vec![Value::from("Max Mustermann")]);

    let (service, socket) = LspService::new(|client| Backend::new(client, scope));
    Server::new(stdin, stdout, socket).serve(service).await;
}
