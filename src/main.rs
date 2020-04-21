use actix_web::{get, web, App, HttpServer, Responder};
use rand::Rng;
use std::f64::consts::PI;
use std::convert::TryFrom;

fn compute(n: u64) -> f64 {
    let mut rng = rand::thread_rng();
    let size = usize::try_from(n).unwrap();
    let mut arr = vec![0.0; size];
    for i in arr.iter_mut() {
        *i = rng.gen::<f64>();
    }
    let mut sum: f64 = 0.0;
    let mut sin_sum: f64 = 0.0;
    for i in arr.into_iter() {
        sum += i;
        sin_sum += (2.0 * PI * i).sin();
    }
    // println!("Sum {}", sum);
    // println!("Sin sum {}", sin_sum);
    sum / sin_sum
}

#[get("/work/{n}")]
async fn index(info: web::Path<u64>) -> impl Responder {
    let n: u64 = info.into_inner();
    println!("Received work: {}", n);
    format!("{}", compute(n))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on :8080");
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
