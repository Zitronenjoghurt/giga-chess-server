use clap::{Parser, Subcommand};
use diesel::{Connection, PgConnection, RunQueryDsl};
use giga_chess_server::database::models::invite_code::{InviteCode, NewInviteCode};
use giga_chess_server::database::schema::invite_codes;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Invites {
        #[command(subcommand)]
        invite_command: InviteCommands,
    },
}

#[derive(Subcommand)]
enum InviteCommands {
    Create { count: u8, comment: Option<String> },
    List,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let mut conn = PgConnection::establish(&database_url).expect("Error connecting to database");

    match cli.command {
        Some(Commands::Invites { invite_command }) => match invite_command {
            InviteCommands::Create { count, comment } => {
                create_invites(&mut conn, count, comment);
            }
            InviteCommands::List => {
                list_invites(&mut conn);
            }
        },
        None => {
            println!("No command specified");
        }
    }
}

fn create_invites(conn: &mut PgConnection, count: u8, comment: Option<String>) {
    let new_invites = (0..count)
        .map(|_| NewInviteCode::new_with_comment(comment.clone()))
        .collect::<Vec<_>>();

    let invites = diesel::insert_into(invite_codes::table)
        .values(&new_invites)
        .get_results::<InviteCode>(conn)
        .unwrap();

    println!("Created {} invite codes:", invites.len());
    invites.iter().for_each(|invite| println!("{}", invite.id));
}

fn list_invites(conn: &mut PgConnection) {
    let invites: Vec<InviteCode> = invite_codes::table
        .load::<InviteCode>(conn)
        .expect("Error loading invite codes")
        .into_iter()
        .filter(|invite| !invite.used)
        .collect();

    if invites.is_empty() {
        println!("No invite codes found");
    } else {
        println!("Found {} unused invite codes:", invites.len());
        for invite in invites {
            println!("{} - {}", invite.id, invite.created_at);
        }
    }
}
