use actix_web::{get, web, App, HttpServer, Responder};
use rand::Rng;

fn compute(n: usize) -> f64 {
    let mut arr = vec![0.0; n];
    let mut sum = 0.0;
    for i in 0..n {
        arr[i] = rand::thread_rng().gen();
        sum += arr[i]
    }
    sum
}

#[get("/work/{n}")]
async fn index(info: web::Path<usize>) -> impl Responder {
    let n: usize = info.into_inner();
    println!("Received work: {}", n);
    format!("{}", compute(n))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on :8080");
    HttpServer::new(|| App::new().service(index))
        .bind("localhost:8080")?
        .run()
        .await
}
