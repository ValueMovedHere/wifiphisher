use clap::Parser;

#[derive(Parser)]
struct Arg {
    #[arg(short, long, value_name = "IFACE_NAME")]
    interface: String,
}
