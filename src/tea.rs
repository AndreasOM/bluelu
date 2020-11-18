use std::process::{Command, Stdio};
use std::io::{Read, Write};

use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use log::{info, trace, warn, debug};

#[derive(Debug)]
pub struct Tea {
	stdout_filename: Option<String>,
	binary: String,
	parameters: Vec<String>,
}

impl Tea {
	pub fn new( binary: &str, parameters: Vec<&str> ) -> Self {
		Self {
			stdout_filename: None,
			binary: binary.into(),
			parameters: parameters.iter().map(|param| param.to_string()).collect(),
		}
	}

	pub fn set_stdout_file( &mut self, stdout_file: &str ) {
		self.stdout_filename = Some( stdout_file.to_string() );
	}

	pub async fn run( &self ) -> anyhow::Result<()> {
//		dbg!( &self);

		let mut command = Command::new( &self.binary );
		for p in &self.parameters {
			command.arg(p);
		}
		command.stdout(Stdio::piped());
//		dbg!(&command);
//		println!("Running:");
		trace!( "Running: {:?}", &command );
//		let o = command.output().expect("fuu");
		let mut child = command.spawn().expect("Error spawning child");

//		child.wait().expect("Error waiting for child");

		let (tx, rx) = channel();

		let stdout = child.stdout.take();
		let thread = tokio::spawn(async move{
			match stdout {
				Some( mut stdout ) => {
				let mut buf = Vec::new();

				trace!("Reading stdout");
				loop {
					let mut byte =[0u8];
					match stdout.read(&mut byte) {
						Ok(0) => {
							break;
						},
						Ok( 1 ) => {
							if byte[0] == 0x0a {
								match String::from_utf8( buf.clone() ) {
									Ok( l ) => {
//										dbg!(&l);
										tx.send( l );
									},
									Err( e ) => {
										warn!("Received non utf-8 character");
										tx.send("Non UTF-8".to_string());
									}
								}
								buf.clear();
							} else {
								buf.push( byte[ 0 ] );
							}
						},
						Ok( _ ) => {
							unreachable!();
						},
						Err(e) => {
							tx.send("Error".to_string());
						},
					};
				}
				},
				None => {
					println!("NO STDOUT");
					warn!("NO STDOUT");
				},
			}
//		} else {
//			println!("NO STDOUT");
//		};


	    	});
//    	        println!("> {}", &buf);

		loop {

			match rx.try_recv() {
				Ok( l ) => {
					println!("{}", l);
				},
				_ => {

				},
			}
			match child.try_wait() {
			    Ok(Some(status)) => {
			    	println!("exited with: {}", status);
			    	break;
			    },
			    Ok(None) => {
//			        println!("status not ready yet, let's really wait");  
			        tokio::time::delay_for( std::time::Duration::from_millis(1000) ).await;
			    },
			    Err(e) => {
			    	println!("error attempting to wait: {}", e);
			    	break;
			    },
			}
		}

		trace!("Done! {:?}", &child );
		thread.await?;

		Ok(())
	}
}