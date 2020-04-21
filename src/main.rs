use actix_web::{get, web, App, HttpServer, Responder};
use rand::Rng;
use std::f64::consts::PI;
use std::convert::TryFrom;

fn compute(mem_size: u64, cpu_size: u64) -> f64 {
    let mut rng = rand::thread_rng();

    // memory work
    let arr_size = usize::try_from(mem_size).unwrap();
    let mut arr = vec![0.0; arr_size];
    for i in arr.iter_mut() {
        *i = rng.gen::<f64>();
    }

    // cpu work
    let max_index = usize::try_from(cpu_size).unwrap();
    let mut sum: f64 = 0.0;
    for i in 0..max_index {
        sum += (2.0 * PI * arr[i]).sin();
    }
    sum
}

#[get("/work/{mem}/{cpu}")]
async fn index(info: web::Path<(u64, u64)>) -> impl Responder {
    //println!("Received work: {}", n);
    format!("{}", compute(info.0, info.1))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on :8080");
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
