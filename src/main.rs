use chrono::{DateTime, Utc};
use futures_util::{SinkExt, StreamExt};
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::rc::Rc;
use std::time::SystemTime;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use ::inject::*;


struct MyService()

impl MyService{
    #[inject]
    fn new() -> Self {
        Self()
    }

    fn hehe(){
        println!("hehe");
    }
}

struct MyService2 {
    myService : MyService
}

impl MyService2 {
    #[inject]
    fn new(myService: MyService) -> Self {
        Self{
            myService : myService
        }
    }

    fn foo(&self){
        MyService::hehe();
    }

}




#[tokio::main]
async fn main() -> std::io::Result<()> {

   let myService = Box::new(MyService());
   let container = container![
        ref myService
    ];

   let instance = get!(&container, MyService2).unwrap();
   instance.foo();

    Ok(())
}
