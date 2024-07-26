// #todo Consider separate command `tan-pm`
// #todo Find a better name: `package`, `pm`, `crate`
// #todo What about new/init?

// tan crate get
// tan crate install
// tan create publish
// tan crate clean
// tan crate upgrade  #question Update or upgrade?
// tan crate list
// tan crate tidy

// #todo Separate `deps` or `dep` command?

// tan deps add/install xxx
// tan deps link (link to a local dependency, for development)
// tan deps remove xxx
// tan deps install/resolve
// tan deps update/upgrade
// tan deps list
// tan deps tidy

// #todo This needs to resolve transitive dependencies!

use clap::ArgMatches;

pub fn handle_deps(deps_matches: &ArgMatches) -> anyhow::Result<()> {
    if let Some(_deps_install_matches) = deps_matches.subcommand_matches("install") {
        // account_orders_list(orders_list_matches).await?;
        println!("DEPS INSTALL TODO");
    } else {
        println!("DEPS TODO");
    }
    Ok(())
}
