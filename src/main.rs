use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /
    let root = warp::path::end().map(|| "Rust National Park");

    warp::serve(root)
        .run(([127, 0, 0, 1], 3030))
        .await;
}