use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;
use std::net::{IpAddr, TcpListener};

async fn health_check() -> HttpResponse{
    println!("Health_Status: UP");
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

/*##########################################################################################################
#*web::get() est equivalent a Route::new().guard(guard::Get)))
#*Ceci veut dire qu c'est un guard qui ne laissera passer au handler SEULEMENT les
#*requete GET
#*
#*Tokio::main est un MACRO. Permet de rendre ma fonction main asynchrone (car techniquement elle ne pourrait pas l'etre)
#*Je peux voir ce qu'il y a dans le macro avant que se soit envoyer au compiler avec la command expand.
#*Je dois utiliser le compilateur nighlty pour ca : cargo +nightly expand
#* En gros on 'expand' le macro && voit le boilerplate derriere
 ##########################################################################################################*/
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let port = listener.local_addr().unwrap().port();
    let ip = listener.local_addr().unwrap().ip();
    log_server(ip,port);

    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check",web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/hello",web::get().to(greet))
            .route("/hello/{name}",web::get().to(greet))
    })
        .listen(listener)?
        .run();

        Ok(server)
}

//BONUS
async fn greet(req: HttpRequest)-> impl Responder{

    let name = req.match_info().get("name").unwrap_or("World");
    /*Le return de la prochaine ligne peut etre retirer si on enleve le semicolon.
     *La derniere expression est le return (mais doit enlever ';')
     *Voir fn healthcheck qui elle n<a pas return ou ';'
     *****************************************************************************/
    return format!("Hello{}!\n", &name);
}
fn log_server(ip: IpAddr, port: u16){
    print!("##################################\n\
    #Addr_iP: {}\n\
    #Port   : {}\n##################################\n\n",ip, port);
}