use clap::Parser;


const BROKER_URI_ENV: &str = "BROKER_URI";
const DEFAULT_EXCHANGE_NAME: &str = "job-manager";
const EXCHANGE_NAME_ENV: &str = "EXCHANGE_NAME";

const DEFAULT_TIMEOUT_ENV: &str = "DEFAULT_TIMEOUT";
const DEFAULT_TIMEOUT: u32 = 5;

const HOST_ENV: &str = "HOST";
const DEFAULT_HOST: &str = "0.0.0.0";

const PORT_ENV: &str = "PORT";
const DEFAULT_PORT: u16 = 8080;


/// A machine learning long running inference platform
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
	/// The service broker uri
	#[arg(short, long, env=BROKER_URI_ENV)]
	pub broker_uri: String,

	/// Exchange name to publish
	#[arg(long, default_value=DEFAULT_EXCHANGE_NAME, env=EXCHANGE_NAME_ENV)]
	pub exchange_name: String,


	/// Default timeout (in seconds) to wait for a response
	#[arg(long, default_value_t=DEFAULT_TIMEOUT, env=DEFAULT_TIMEOUT_ENV)]
	pub default_timeout: u32,

	/// Host to listen HTTP requests
	#[arg(long, default_value=DEFAULT_HOST, env=HOST_ENV)]
	pub host: String,

	/// Port to listen http requests
	#[arg(long, default_value_t=DEFAULT_PORT, env=PORT_ENV)]
	pub port: u16,

}

impl Arguments {
	pub fn load() -> Self {
		Arguments::parse()
	}
}