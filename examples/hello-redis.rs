//Following https://tokio.rs/tokio/tutorial

//Introduction

//mini-redis-cli get foo

use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    //open a connection to the mini reds address
    let mut client = client::connect("127.0.0.1:6379").await?;

    //set the key "hello" with value "world"
    client.set("Hello","world!".into()).await?;

    //get key "hello"
    let result = client.get("hello").await?;
    
    println!("Got value from the server; result={:?}",result);

    Ok(())
}

// async fn say_world() {
//     println!("world");
// }

// #[tokio::main]
// async fn main() {
//     // Calling `say_world()` does not execute the body of `say_world()`.
//     let op = say_world();

//     // This println! comes first
//     println!("hello");

//     // Calling `.await` on `op` starts executing `say_world`.
//     op.await;
// }