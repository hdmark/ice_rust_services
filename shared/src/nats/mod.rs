extern crate dotenv;
pub mod models;
use dotenv::dotenv;
use nats;
use std::env;

pub type NatsConnection = nats::Connection;

pub fn establish_nats_connection() -> NatsConnection {
  dotenv().ok();

  let nats_url = env::var("NATS_URL").expect("NATS_URL must be set");
  let nats_user = env::var("NATS_USER").expect("NATS_USER must be set");
  let nats_pass = env::var("NATS_PASSWORD").expect("NATS_PASSWORD must be set");

  nats::ConnectionOptions::new()
    .with_user_pass(&nats_user, &nats_pass)
    .connect(&nats_url)
    .expect("Nats connection failed")

  //nats::connect(&nats_url).expect("Nats Connection failed")
}
