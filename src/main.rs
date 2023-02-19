/*##########################################################################################################
#* RUN : cet function existe dans :
#*       src/lib.rs
#*donc la prochain ligne est son import
##########################################################################################################*/
use std::net::{TcpListener};
use newsletter::startup::run;

/*##########################################################################################################
#*Tokio::main est un MACRO. Permet de rendre ma fonction main asynchrone (car techniquement elle ne pourrait pas l'etre)
#*Je peux voir ce qu<il y a dans le macro avant que se soit envoyer au compiler avec la command expand.
#*Je dois utiliser le compilateur nighlty pour ca : cargo +nightly expand
#* En gros on 'expand' le macro && voit le boilerplate derriere
 ##########################################################################################################*/
#[tokio::main]
async fn main() -> std::io::Result<()> {
    run(get_port())?.await
}

fn get_port()-> TcpListener {
    TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port")
}


