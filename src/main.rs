use actix_web::{get,  App, HttpResponse, HttpServer, Responder};


#[get("/health")]
async fn current_context() -> impl Responder {

    let mut context = String::from("Current context:\r\n");
    let username = String::from("   username: ") + &whoami::username() + &String::from("\r\n") ;
    context.push_str(&username);
    let realname = String::from("   realname: ") + &whoami::realname() + &String::from("\r\n");
    context.push_str(&realname);
    let devicename = String::from("   devicename: ") + &whoami::devicename() + &String::from("\r\n");
    context.push_str(&devicename);
    let hostname = String::from("   hostname: ") + &whoami::hostname() + &String::from("\r\n");
    context.push_str(&hostname);
    let distro = String::from("   distro: ") + &whoami::distro() + &String::from("\r\n");
    context.push_str(&distro);

    let ip = local_ipaddress::get();
    let ipstr = ip.as_deref().unwrap_or("   No ip address detected!");
    
    let ipaddress = String::from("   ipaddress: ") + &ipstr + &String::from("\r\n");
    context.push_str(&ipaddress);
 
    HttpResponse::Ok().body(context)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(current_context)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
