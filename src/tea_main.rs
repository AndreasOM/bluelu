use clap::{Arg, App};

use bluelu::tea::Tea;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let matches = App::new("tea")
						.version("0.1")
						.author("Andreas N. <bluelu@omni-mad.com>")
						.about("Classic unix tee on steroids")
						.arg(Arg::with_name("stdout-file")
							.help("Sets file used for stdout")
							.short("o")
							.long("stdout-file")
							.value_name("STDOUT-FILE")
							.required(false)
							.takes_value(true)
						)
						.arg(Arg::with_name("binary")
							.help("Sets the binary to run")
							.short("b")
							.long("binary")
							.value_name("BINARY")
							.required(true)
						)
						.arg(Arg::with_name("parmeters")
							.help("Sets the parameters")
//							.short("p")
//							.long("parameters")
//							.value_name("PARAMETERS")
							.required(false)
							.multiple(true)
							.last(true)
						).get_matches();
	dbg!(&matches);

	if matches.is_present("binary") {
		let binary = matches.value_of("binary").unwrap();
//		let params = matches.values_of("parmeters").unwrap().collect::<Vec<_>>();//Vec::new();
		let params = match matches.values_of("parmeters") {
			Some(v) => v.collect::<Vec<_>>(),//Vec::new();
			_ => Vec::new(),
		};

		let mut cmd = Tea::new( binary, params );
		if let Some( stdout_filename ) = matches.value_of("stdout-file") {
			cmd.set_stdout_file( stdout_filename );
		};
		cmd.run().await?;
	}
	Ok(())
}
