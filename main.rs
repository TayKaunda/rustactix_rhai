use std::ops::DivAssign;
use actix_web::{
    HttpServer,
    get,
    App,
    web::Path,
    Responder,
};
use rhai::Engine;

#[get("/multiply/{num1}/{num2}/{num3}")]
async fn multiply(path: Path<(i64, i64, i64)>) -> impl Responder {
    //get the numbers from the url path
    let (num1, num2, num3) = path.into_inner();

    //create rhai engine instance
    let mut engine = Engine::new();

    //register Api to interact with rhai
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);
    engine.register_fn("num3", move || num3);

    //run script
    let result = engine.eval_file::<i64>("src/multiply.rhai".into()).unwrap();

    //return a result
    format!("{result}")
}

#[get("/add/{num1}/{num2}/{num3}")]
async fn add(path: Path<(i64, i64, i64)>) -> impl Responder {
    let (num1, num2, num3) = path.into_inner();
    let result = num1 + num2 + num3;
    format!("{result}")
}

#[get("/divide/{num1}/{num2}/{num3}")]
async fn divide(path: Path<(i64, i64, i64)>) -> impl Responder {
    let (num1, num2, num3) = path.into_inner();
    if num3 == 0 {
        "Error: Division by zero".to_string()
    } else {
        let result = num1 / num2;
        format!("{result}")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(multiply)
            .service(add)
            .service(divide)
    })
    .bind(("127.0.0.1", 7000))?
    .run()
    .await
}