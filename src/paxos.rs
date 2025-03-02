use std::net::{IpAddr, Ipv4Addr};

use axum::{extract::Path, routing::get, Router};

use crate::net::{AxumListener, TcpListener};

pub struct PaxosRunArgs {
    pub port: u16,
    //pub persistance_dir: Utf8PathBuf,
}

// Hosts need to persist some info in between crashes, this can be done via
// disk on the host. In sim we'll need a tempdir that lasts for the simulation

pub async fn run_paxos(args: PaxosRunArgs) -> anyhow::Result<()> {
    // What do we need to do?
    // - Bind to a port
    // - Accept requests (lets use axum)
    // - Run the paxos protocol based on those requests
    //   - We're can reuse the same server for client requests (non paxos members)
    //     and paxos members. We'll assume that we could switch these to secure
    //     down the line.

    let router = Router::new().route(
        "/greet/{name}",
        get(|Path(name): Path<String>| async move { format!("Hello {name}!") }),
    );
    let listener = TcpListener::bind((IpAddr::from(Ipv4Addr::UNSPECIFIED), args.port)).await?;

    axum::serve(AxumListener(listener), router).await?;

    Ok(())
}
