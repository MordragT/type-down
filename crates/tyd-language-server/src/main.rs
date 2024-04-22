use async_std::task;
use backend::Backend;
use tower_lsp::{LspService, Server};

pub mod backend;
pub mod kind;
pub mod token;
pub mod tree;

fn main() {
    task::block_on(run())
}

async fn run() {
    let stdin = async_std::io::stdin();
    let stdout = async_std::io::stdout();

    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
