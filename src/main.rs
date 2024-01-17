use clap::{Arg, ArgAction, Command};

mod errors;
mod exercise;
mod user_profile;
mod utils;
mod set;
mod workout_session;

fn cli() -> Command {
    Command::new("workout")
        .about("Controls a workout session")
        .subcommand_required(true)
        .arg_required_else_help(true)
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
            Command::new("current-profile").about("Shows the current user profile"),
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
            Command::new("start").about("Starts a workout session"),
            Command::new("choose")
                .about("Chooses a workout session")
                .arg(
                    Arg::new("name")
                        .help("The name of the workout session")
                        .required(true)
                        .index(1),
                ),
            Command::new("current-session").about("Shows the current workout session"),
            Command::new("list").about("Lists all workout sessions"),
            Command::new("delete").about("Deletes a chosen workout session"),
        ])
        .subcommands([
            Command::new("create-exercise")
                .about("Creates an exercise")
                .arg(
                    Arg::new("name")
                        .help("The name of the exercise")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("description")
                        .help("The description of the exercise")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::new("musclegroups")
                        .help("The muscle groups of the exercise")
                        .required(true)
                        .index(3)
                        .action(ArgAction::Append),
                ),
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
        Some(("choose", sub_m)) => {
            let name = sub_m.get_one::<String>("name");
            workout_session::choose(name.unwrap().to_string());
        }
        Some(("current-session", _)) => {
            let workout_session = workout_session::get_current_session();
            println!(
                "Current workout session: {}",
                workout_session.get_name().to_string()
            );
        }
        Some(("list", _)) => {
            workout_session::list();
        }
        Some(("delete", _)) => {
            workout_session::delete();
        }
        Some(("create-profile", sub_m)) => {
            let name = sub_m.get_one::<String>("name");
            user_profile::create_profile(name.unwrap().to_string()).unwrap();
            user_profile::choose_profile(name.unwrap().to_string()).unwrap();
        }
        Some(("list-profiles", _)) => {
            let users = user_profile::read_profiles().expect("Unable to read user profiles");
            for user in users.list() {
                println!("{}", user.get_name());
            }
        }
        Some(("delete-profile", sub_m)) => {
            let name = sub_m.get_one::<String>("name");
            user_profile::delete_profile(name.unwrap().to_string()).unwrap();
            println!("Deleted profile: {}", name.unwrap());
            user_profile::choose_profile("default".to_string()).unwrap();
        }
        Some(("choose-profile", sub_m)) => {
            let name = sub_m.get_one::<String>("name");
            user_profile::choose_profile(name.unwrap().to_string()).unwrap();
            println!("Chosen profile {}", name.unwrap());
        }
        Some(("current-profile", _)) => {
            let user = user_profile::read_current_user().unwrap();
            println!("Current user: {}", user);
        }
        Some(("create-exercise", _)) => {
        }
        _ => println!("Invalid command"),
    }
}
