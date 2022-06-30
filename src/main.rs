use rweb::{hyper::StatusCode, *};
use std::{collections::HashMap, convert::Infallible};

/// a fallible endpoint that fails randomly half the time
#[get("/fallible")]
async fn fallible() -> Result<Box<dyn Reply>, Infallible> {
    generic_fallible(async {
        // this async block just needs to return anyhow::Result<R>, where R is some type that implements Reply
        if fastrand::bool() {
            anyhow::bail!("oh no I FAILed!")
        }
        Ok(String::from("this returns 200 with text/plain mime type"))
    })
    .await
}

async fn generic_fallible<R: Reply + 'static>(
    f: impl Future<Output = anyhow::Result<R>>,
) -> DynReply {
    match f.await {
        Ok(res) => Ok(Box::new(res)),
        Err(err) => {
            let mut map = HashMap::new();
            map.insert("error", err.to_string());
            Ok(Box::new(rweb::reply::with_status(
                rweb::reply::json(&map),
                StatusCode::INTERNAL_SERVER_ERROR,
            )))
        }
    }
}

type DynReply = Result<Box<dyn Reply>, Infallible>;

#[tokio::main]
async fn main() {
    serve(fallible()).run(([127, 0, 0, 1], 3030)).await;
}
