use crate::client;
use clap::{clap_app, crate_version, App};

fn app() -> App<'static, 'static> {
    clap_app!(bisq_cli =>
        (version: crate_version!())
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        (@subcommand get_version =>
          (about: "get the version of running bisq daemon")
          (name: "get-version")
        )
    )
}

pub async fn run() {
    let matches = app().get_matches();
    match matches.subcommand() {
        ("get-version", Some(_)) => get_version(),
        _ => unreachable!(),
    }.await
}

async fn get_version() {
    match client::get_version().await {
        Err(e) => {
            eprintln!("Error getting version: {:?}", e);
            std::process::exit(1)
        }
        Ok(version) => println!("v{}",version)
    }
}
