use axum::{body::Body, extract::Request};
use http_body_util::BodyExt;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use paxrs::{
    net::connector,
    paxos::{run_paxos, PaxosRunArgs},
};

#[test]
fn test_server() -> turmoil::Result<()> {
    let mut sim = turmoil::Builder::default().build();
    sim.host("server", || async move {
        let args = PaxosRunArgs { port: 8080 };
        run_paxos(args).await?;
        Ok(())
    });

    sim.client("client", async move {
        let client = Client::builder(TokioExecutor::new()).build(connector::connector());

        let mut request = Request::new(Body::empty());
        *request.uri_mut() = hyper::Uri::from_static("http://server:8080/greet/foo");
        let res = client.request(request).await?;

        let (parts, body) = res.into_parts();
        let body = body.collect().await?.to_bytes();
        let res = hyper::Response::from_parts(parts, body);

        tracing::info!("Got response: {:?}", res);
        Ok(())
    });

    sim.run()
}
