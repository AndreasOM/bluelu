use clap::Parser;

use bluelu::tea::Tea;
use log::info;

#[derive(Parser)]
#[command(name = "tea")]
#[command(version = "0.1")]
#[command(author = "Andreas N. <bluelu@omni-mad.com>")]
#[command(about = "Classic unix tee on steroids")]
struct CliArgs {
    /// Sets file used for stdout
    #[arg(short = 'o', long = "stdout-file", value_name = "STDOUT-FILE")]
    stdout_file: Option<String>,

    /// Sets the binary to run
    #[arg(short = 'b', long = "binary", value_name = "BINARY", required = true)]
    binary: String,

    /// Sets the parameters
    #[arg(trailing_var_arg = true)]
    parameters: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!("Starting...");

    let args = CliArgs::parse();

    let mut cmd = Tea::new(
        &args.binary,
        args.parameters.iter().map(|s| s.as_str()).collect(),
    );
    if let Some(stdout_filename) = &args.stdout_file {
        cmd.set_stdout_file(stdout_filename);
    }
    cmd.run().await?;

    Ok(())
}
