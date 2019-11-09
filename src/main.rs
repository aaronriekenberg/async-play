use async_std::task;

use futures::try_join;

use log::info;

async fn make_http_call(uri: &str) -> Result<String, surf::Exception> {
    info!("make_http_call uri: {}", uri);

    surf::get(uri).recv_string().await
}

async fn make_http_calls() -> Result<(), surf::Exception> {
    let uri1 = "https://httpbin.org/get";
    let future1 = make_http_call(&uri1);

    let uri2 = "https://httpbin.org/get2";
    let future2 = make_http_call(&uri2);

    info!("before try_join");

    let results = try_join!(future1, future2);

    info!("make_http_calls got results: {:#?}", results);

    Ok(())
}

fn main() -> Result<(), surf::Exception> {
    femme::start(log::LevelFilter::Info)?;

    let future = make_http_calls();

    task::block_on(future)
}
