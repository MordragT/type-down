use async_std::task;
use backend::Backend;
use tower_lsp::{LspService, Server};
use tyd_eval::{builtin, value::Value};
use tyd_pandoc::{plugin, visitor::PandocVisitor};

pub mod backend;
pub mod semantic;
pub mod syntax;

fn main() {
    task::block_on(run())
}

async fn run() {
    let stdin = async_std::io::stdin();
    let stdout = async_std::io::stdout();

    let scope = plugin::plugin()
        .into_scope()
        .register_symbol("title", "Default title")
        .register_symbol("author", vec![Value::from("Max Mustermann")])
        // Builtins
        .register_func("let", builtin::Let)
        .register_func("List", builtin::List)
        .register_func("Map", builtin::Map);

    let (service, socket) = LspService::new(|client| Backend::new(client, scope, PandocVisitor {}));
    Server::new(stdin, stdout, socket).serve(service).await;
}
