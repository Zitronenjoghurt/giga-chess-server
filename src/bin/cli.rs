use clap::{Parser, Subcommand};
use diesel::{Connection, PgConnection, RunQueryDsl};
use giga_chess_server::app::security::hash_bytes;
use giga_chess_server::database::models::user::{NewUser, User};
use giga_chess_server::database::schema::users;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    CreateUser { name: String },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::CreateUser { name }) => {
            let (username, token) = create_user(name.clone());
            println!(
                "Created user:\nName: {username}\nToken: {token}\n\nOnly share this token with the assigned user, be a trusty admin and delete it after you're done."
            );
        }
        None => {
            println!("No command specified");
        }
    }
}

fn create_user(name: String) -> (String, String) {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let mut connection =
        PgConnection::establish(&database_url).expect("Error connecting to database");

    let token = uuid::Uuid::new_v4().to_string();
    let token_hash = hash_bytes(token.as_bytes()).unwrap();

    let user = NewUser::new(&name, &token_hash);

    let user = diesel::insert_into(users::table)
        .values(user)
        .get_result::<User>(&mut connection)
        .unwrap();

    (user.name, token)
}
