// #![deny(warnings)]

// use std::sync::{
//     atomic::{AtomicUsize, Ordering},
//     Arc,
// };

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Response, Server};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let addr = ([127, 0, 0, 1], 3000).into();

    // For the most basic of state, we just share a counter, that increments
    // with each request, and we send its value back in the response.
    // let counter = Arc::new(AtomicUsize::new(0));
    let mut counter: u32 = 0;

    // The closure inside `make_service_fn` is run for each connection,
    // creating a 'service' to handle requests for that specific connection.
    let make_service = make_service_fn(move |_| {
        // While the state was moved into the make_service closure,
        // we need to clone it here because this closure is called
        // once for every connection.
        //
        // Each connection could send multiple requests, so
        // the `Service` needs a clone to handle later requests.
        // let counter = counter.clone();

        async move {
            // This is the `Service` that will handle the connection.
            // `service_fn` is a helper to convert a function that
            // returns a Response into a `Service`.
            Ok::<_, Error>(service_fn(move |_req| {
                // Get the current count, and also increment by 1, in a single
                // atomic operation.
                counter += 1;
                let resp = format!("Goodbye Hello World: {}", counter);
                async move { Ok::<_, Error>(Response::new(Body::from(resp))) }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
