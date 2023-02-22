use actix_web::{HttpRequest, Responder};

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    /*Le return de la prochaine ligne peut etre retirer si on enleve le semicolon.
     *La derniere expression est le return (mais doit enlever ';')
     *Voir fn healthcheck qui elle n<a pas return ou ';'
     *****************************************************************************/
    return format!("Hello{}!\n", &name);
}
