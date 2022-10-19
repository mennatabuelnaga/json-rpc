// Copyright 2019-2021 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.


// cargo run --bin jsonrpsee_server

use std::net::SocketAddr;

use jsonrpsee::server::ServerBuilder;
use jsonrpsee::RpcModule;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let filter = tracing_subscriber::EnvFilter::try_from_default_env()?
		.add_directive("jsonrpsee[method_call{name = \"say_hello\"}]=trace".parse()?);

	tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).finish().try_init()?;
	let addr = run_server().await?;
	let url = format!("ws://{}", addr);
    println!("server started on url: {:?}", url);

	// How do we keep the server running without using an infinite loop?
    loop {
        
    } 

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
   
	let server = ServerBuilder::default().build("127.0.0.1:12345").await?;
	let mut module = RpcModule::new(());
    module.register_method("echo", |params, _| {
        let n: String = params.one()?;
        Ok(n)
    })
    .unwrap();

	let addr = server.local_addr()?;
	let handle = server.start(module)?;
	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.clone().stopped());

	Ok(addr)
}

