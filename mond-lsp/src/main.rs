#[tokio::main]
async fn main() {
    mond_lsp::serve(tokio::io::stdin(), tokio::io::stdout()).await;
}
