use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::time::Duration;
use async_std::task;
use std::{thread, time};
use local_ip_address::local_ip;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let my_local_ip = local_ip().unwrap();
    println!("This is my local IP address: {:?}", my_local_ip);
    HttpServer::new(|| {
        App::new()
            .service(get_endpoint_for_cputask)
            .service(get_endpoint_for_io_task)
            .service(get_endpoint)
    })
    .bind(my_local_ip + ":8080")?
    .run()
    .await
}

#[get("/get-endpoint")]
async fn get_endpoint() -> impl Responder {
    HttpResponse::Ok().body("Get Successfull !")
}


#[get("/cpu-intense-task")]
async fn get_endpoint_for_cputask() -> impl Responder {
    run_cpu_intense_work();
    HttpResponse::Ok().body("CPU Intense Task Completed.")
}


#[get("/high-io-task")]
async fn get_endpoint_for_io_task() -> impl Responder {
    task::sleep(Duration::from_millis(10)).await;
    HttpResponse::Ok().body("I/O Task Completed.")
}


async fn run_cpu_intense_work() {
    let mut count = 0u32;
    loop {
        run_math_computations();
        count += 1;
        if count == 1_000_000 {
            break;
        }
    }
}

async fn run_math_computations() {
    let f = 123456789.987654321_f64;
    let d = f.atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan();
    d.cbrt();
}
