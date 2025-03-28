use warp::Filter;

pub async fn start_api_server(port: u16) {
    let routes = warp::path!("balance" / String)
        .map(|address: String| {
            format!("Balance for {}: 100\n", address) // Placeholder
        });
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
