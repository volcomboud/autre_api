/*##########################################################################################################
#* RUN : cet function existe dans :
#*       src/lib.rs
#*     donc on import
##########################################################################################################*/
use newsletter::configuration::get_configuration;
use newsletter::startup::run;
use std::net::TcpListener;

/*##########################################################################################################
#*Tokio::main est un MACRO. Permet de rendre ma fonction main asynchrone (car techniquement elle ne pourrait pas l'etre)
#*Je peux voir ce qu<il y a dans le macro avant que se soit envoyer au compiler avec la command expand.
#*Je dois utiliser le compilateur nighlty pour ca : cargo +nightly expand
#* En gros on 'expand' le macro && voit le boilerplate derriere
 ##########################################################################################################*/
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
