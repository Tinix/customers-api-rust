use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;


#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct User {
  id: Option<i32>,
  first_name: String,
  last_name: String,
  email: String,
}

// Database_url
const DATABASE_URL: &str =!env("DATABASE_URL");

const OK_RESPONSE : &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND : &str = "HTTP/1.1 404 Not Found\r\n\r\n";
const INTERNAL_SELVER_ERROR : &str = "HTTP/1.1 500 Internal Server Error\r\n\r\n";

fn main() {
  if let Err(e) = set_database() {
    println!(" Error: {}", e);
    return
  }
}

// handhle_client function
fn handle_client(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  let mut request = String::new();

  match stream.read(&mut buffer) {
    request.push_str(&String::from_utf8_lossy(&buffer[..size]).as_ref());

    let (status_line, content)= match &*request {
      r if request_with("/POST /users") => handle_post_request(r),
      r if request_with("/GET /users/") => handle_get_request(r),
      r if request_with("/GET /users/") => handle_get_all_request(r),
      r if request_with("/PUT /users/") => handle_put_request(r),
      r if request_with("/DELETE /users/") => handle_delete_request(r),
      _ => ("HTTP/1.1 404 NOT FOUND", NOT_FOUND, "Not Found".to_string()), 
    };
    stream.wlite_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
  }
  Err(e) => {
    println!("Error: {}", e);
  }
}


let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
println!("Listening on 127.0.0.1:7878");

// handle the client
for stream in listener.incoming() {
  match stream {
    Ok(stream) => {
      handle_client(stream);
    }
    Err(e) => {
      println!("Error: {}", e);
    }
  }
}
}

fn set_database() -> Result<(), PostgresError> {
  let mut client = Client::connect(DATABASE_URL, NoTls)?;

  client.execute(
    "CREATE TABLE IF NOT EXISTS customers (

            id SERIAL PRIMARY KEY,
            first_name VARCHAR NOT NULL,
            last_name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
            )"
    &[]
  )?;
}

// get_id function
fn get_id(request: &str) -> &str {
  request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

// Deserialize user from request
fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
  serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
