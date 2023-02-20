use std::net::TcpListener;
use sqlx::{Connection, PgConnection};

//Local Crates
use newsletter::startup::run;
use newsletter::configuration::get_configuration;

/*##########################################################################################################
#*[Tokio::test] -> equivalent a [tokio::main]. Une macro accessible avec expand
#*Commande : "cargo +nightly expand --test health_check" (health_check == name of file)
#*
#*Utiliser tokio::test nous evite d'avoir a entrer l'attributs de test (#[test])
#*
#*       src/lib.rs
#*donc la prochain ligne est son import
##########################################################################################################*/

#[tokio::test]
async fn health_check_works(){
    //arrangement
    let address = spawn_app();
    //Utiliser reqwest pour envoyer les requete Http a l'app
    let client = reqwest::Client::new();

    //L'action
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("failure to execute request.");

    //Assert -- Reponse = 200 && n'a pas de body
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
//Cette fn va launch le app en background
/*##########################################################################################################
#*tokio::spawn
#*documentation : when a tokio runtime is shutdown all tasks spawned on it are dropped. tokio::test spins up
#*a new runtime at the beginning of each test case and they shut down at the end of each test case.
#*In other words, good news - no need to implement any clean up logic to avoid leaking resources between test
#*runs.
#* [PORT]->127.0.0.1:0     Le 0 indique que le port sera scan && attribuer par le OS
##########################################################################################################*/
fn spawn_app()-> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");

    //Fetch le PORT attribuer par le systeme
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

/*##########################################################################################################
#*
#*the keys and values [in our form] are encoded in key-value tuples separated by ‘&’, with a ‘=’ between
#*the key and the value. Non-alphanumeric characters in both keys and values are percent encoded.
#*
#*For example: if the name is Le Guin and the email is ursula_le_guin@gmail.com the POST request body
#*should be name=le%20guin&email=ursula_le_guin%40gmail.com (spaces are replaced by %20 while "@" becomes "%40"
#* a reference conversion table can be found w3cschools’s website).
##########################################################################################################*/

#[tokio::test]
async fn subscribe_return_200_for_valid_form_data() {
    //arrangement-----------------------------------------------------------------
    let app_address = spawn_app();
    //dealing with database config
    let configuration = get_configuration().expect("Failed to read configuration file");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");
    //end db config

    let client = reqwest::Client::new();

    //L'action--------------------------------------------------------------------
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert ------------------------------------------------------------------------
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_return_400_when_data_is_missing() {
    //arrangement
   let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_case = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing the email and the name")
    ];

    for(invalid_body, error_message) in test_case {
        //L'action
        let response = client
            .post(format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        //Assert --
        assert_eq!(
            400,
            response.status().as_u16(),
            //Msg d'erreur custom pour donner plus d'info sur le test; Comme je passe un test parametrer invalide, je veux savoir
            //quel portion du Vector est en failure
            "The APi did not fail with code:400 BadRequest when the payload was {}.", error_message
        );
    }
}
