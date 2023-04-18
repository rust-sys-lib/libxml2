use clap::Parser;

mod api;
mod cli;

fn main() {
    let cli = cli::Cli::parse();

    let api = api::Api::load();

    let out_dir = cli.out.into_inner();

    for file in api.files.items.iter().filter(|x| x.deprecated.is_none()) {
        use inflections::Inflect;
        println!("pub mod {};", file.name.to_snake_case());
    }

    println!("{:?}", out_dir);
}
