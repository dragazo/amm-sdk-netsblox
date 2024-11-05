use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Path to the input composition file
    path: String,

    /// Print the composition instead of the generated NetsBlox project
    #[clap(short, long)]
    composition: bool,

    /// Use pretty print mode if applicable
    #[clap(short, long)]
    pretty: bool,
}

fn main() {
    let args = Args::parse();

    let content = std::fs::read(&args.path).unwrap();

    let composition = match args.path.rsplit('.').next().unwrap_or_default() {
        "musicxml" => amm_sdk::storage::Storage::MusicXML.load_data(&content).unwrap(),
        "mid" | "smf" => amm_sdk::storage::Storage::MIDI.load_data(&content).unwrap(),
        _ => amm_sdk::storage::Storage::AMM.load_data(&content).unwrap(),
    };

    match args.composition {
        true => match args.pretty {
            true => println!("{composition:#?}"),
            false => println!("{composition:?}"),
        }
        false => println!("{}", amm_sdk_netsblox::translate(&composition).unwrap()),
    }
}
