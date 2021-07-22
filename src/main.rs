use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::time::Duration;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(cpu-intense-task)
            .service(high-io-task)
    })
    .bind("")?
    .run()
    .await
}

#[get("/cpu-intense-task")]
async fn getEndpointForCPUTask() -> impl Responder {
    runCpuIntenseWork();
    HttpResponse::Ok().body("CPU Intense Task Completed.")
}


#[get("/high-io-task")]
async fn getEndpointForIoTask() -> impl Responder {
    task::sleep(Duration::from_millis(10)).await;
    HttpResponse::Ok().body("I/O Task Completed.")
}


async fn runCpuIntenseWork() {
    let mut count = 0u32;
    loop {
        runMathComputations();
        count += 1;
        if count == 1_000_000 {
            break;
        }
    }
}

async fn runMathComputations() {
    let f = 123456789.987654321_f64;
    let d = f.atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan();
    d.cbrt();
}