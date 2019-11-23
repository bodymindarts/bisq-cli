use crate::bisq;
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

pub fn run() {
    let matches = app().get_matches();
    match matches.subcommand() {
        ("get-version", Some(_)) => bisq::get_version(),
        _ => unreachable!(),
    }
}
