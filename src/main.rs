use no_name::run;

// for Web support I use async functions. Tokio makes it work with native.
#[tokio::main]
async fn main() {
    
    run().await;
}
