use clap::Clap;

use codegen_for_async_graphql_renderer::{generate_from_path, Config};

#[derive(Clap)]
#[clap()]
struct Opts {
    _dummy: Option<String>,
    #[clap(short, long, required = true)]
    schema: String,
    #[clap(short, long, required = true)]
    output: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let path = opts.schema;
    let config = Config {
        output_bnase_path: opts.output,
    };
    generate_from_path(&path, &config);
}
