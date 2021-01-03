#![deny(warnings)]

use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode, Method};

use ssh2::Session;
use std::error::Error;

use std::io::Read;
use rayon::prelude::*;

use std::net::{ TcpStream };
use config::{Config, File, FileFormat};
use std::net::SocketAddr;

struct APConfig {
    username: String,
    password: String,
    hosts:    Vec<String>,
    address:  String,
    command:  String,
}

const RESPONSE: &str = r##"
<!DOCTYPE html>
<html>
  <head>
    <META HTTP-EQUIV="Content-Type" CONTENT="text/html; charset=UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />

    <style type="text/css">
      body {
        width: 15em;
        margin: 0 auto;
        font-family: "Lucida Console", Monaco, monospace;
      }
    </style>
  </head>
  <body>
    <h1>0 OK, 0:1</h1>
  </body>
</html>
"##;

async fn hello(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match(req.method(), req.uri().path()) {
        (&Method::GET, "/inform") => {
            do_work();
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "text/html")
                .header("server", "hyper")
                .body(Body::from(RESPONSE)).unwrap();
            Ok(response)
        },
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
    //Ok(Response::new(Body::from(RESPONSE)))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let cfg = APConfig::read_config();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let socket_addr:SocketAddr = cfg.address.parse().expect("Unble to parse socket address");
    let server = Server::bind(&socket_addr).serve(make_svc);

    println!("Listening on http://{}", &cfg.address);

    server.await?;

    Ok(())
}

fn do_work() {
    let cfg = APConfig::read_config();
    cfg.hosts.par_iter()
        .for_each(|host|
        if let Err(e) = invoke_inform(&host, &cfg.username, &cfg.password, &cfg.command) {
            println!("AP {}: {}", &host, e);
        }
    );
}

impl APConfig {
    fn read_config() -> APConfig {
        let mut c = Config::new();
        c.merge(File::new("settings", FileFormat::Toml).required(true)).unwrap();
        let username = c.get_str("user").unwrap();
        let password = c.get_str("password").unwrap();
        let address  = c.get_str("address").unwrap();
        let command  = c.get_str("command").unwrap();

        let host_values = c.get_array("hosts").unwrap();
        let mut hosts: Vec<String> = Vec::with_capacity(3);
        for h in host_values {
            hosts.push(h.into_str().unwrap());
        }
        APConfig {
            username,
            password,
            hosts,
            address,
            command,
        }
    }
}

fn invoke_inform(host: &str, username: &str, password: &str, command: &str) -> Result<String, Box<dyn Error>> {
    let tcp = TcpStream::connect((host, 22))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_password(username, password)?;

    let mut channel = sess.channel_session()?;
    channel.exec(&command)?;
    let mut s = String::new();
    channel.read_to_string(&mut s)?;
    print!("{}{}", host, s);
    channel.wait_close()?;
    // println!("{}", channel.exit_status()?);
    Ok(s)
}

