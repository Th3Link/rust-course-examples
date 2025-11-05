use rusty_the_robot::run::run; // - in crate name translates to _

#[tokio::main]
async fn main() {
    run().await;
}
