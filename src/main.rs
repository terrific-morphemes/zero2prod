use actix_web::dev::Server;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    run()?.await
}
