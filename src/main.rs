use clap::{Arg, ArgAction, Command};

mod exercise;
mod user_profile;
mod utils;
mod workout_session;

fn cli() -> Command {
    Command::new("workout")
        .about("Controls a workout session")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([
            Command::new("start").about("Starts a workout session"),
            Command::new("end").about("Ends a workout session"),
            Command::new("choose").about("Chooses a workout session"),
            Command::new("list").about("Lists all workout sessions"),
            Command::new("delete").about("Deletes a workout session"),
            Command::new("edit").about("Edits a workout session"),
        ])
        .subcommands([
            Command::new("create-profile")
                .about("Creates a user profile")
                .arg(
                    Arg::new("name")
                        .help("The name of the user profile")
                        .required(true)
                        .action(ArgAction::Set)
                        .index(1),
                ),
            Command::new("list-profiles").about("Lists all user profiles"),
            Command::new("choose-profile")
                .about("Chooses a user profile")
                .arg(
                    Arg::new("name")
                        .help("The name of the user profile")
                        .required(true)
                        .action(ArgAction::Set)
                        .index(1),
                ),
            Command::new("delete-profile")
                .about("Deletes a user profile")
                .arg(
                    Arg::new("name")
                        .help("The name of the user profile")
                        .required(true)
                        .action(ArgAction::Set)
                        .index(1),
                ),
        ])
        .subcommands([
            Command::new("create-exercise").about("Creates an exercise"),
            Command::new("list-exercises").about("Lists all exercises"),
            Command::new("delete-exercise").about("Deletes an exercise"),
            Command::new("edit-exercise").about("Edits an exercise"),
            Command::new("choose-exercise").about("Chooses an exercise"),
        ])
        .subcommands([Command::new("create-set")
            .about("Creates a set for the current workout session")
            .arg(
                Arg::new("reps")
                    .help("The number of reps")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("weight")
                    .help("The weight lifted")
                    .required(true)
                    .index(2),
            )])
}

fn main() {
    let binding = cli();
    let matches = binding.get_matches();
    match matches.subcommand() {
        Some(("start", _)) => workout_session::start(),
        Some(("end", _)) => workout_session::end(),
        Some(("create-profile", sub_m)) => {
            let name = sub_m.get_one::<String>("name");
            user_profile::create_profile(name.unwrap().to_string());
            println!("Creating user profile");
        }
        Some(("list-profiles", _)) => {
            let users = user_profile::read_profiles().expect("Unable to read user profiles");
            for user in users.list() {
                println!("{}", user.get_name());
            }
        },
        Some(("delete-profile", sub_m)) => {
            let name = sub_m.get_one::<String>("name");
            user_profile::delete_profile(name.unwrap().to_string());
            println!("Deleting user profile");
        }
        _ => println!("Invalid command"),
    }
}
