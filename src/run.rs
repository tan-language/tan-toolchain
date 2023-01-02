use clap::ArgMatches;
use tan::eval::{env::Env, prelude::setup_prelude};

use crate::util::eval_string;

// #TODO properly implement this.
pub fn handle_run(run_matches: &ArgMatches) -> anyhow::Result<()> {
    let path: &String = run_matches
        .get_one("PATH")
        .expect("missing path to program file");

    let input = std::fs::read_to_string(path).expect("cannot read input");

    let mut env = setup_prelude(Env::default());

    eval_string(&input, &mut env);

    Ok(())
}
