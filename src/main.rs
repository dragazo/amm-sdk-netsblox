fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("usage: {} [input path]", args[0]);
    }

    let content = std::fs::read(&args[1]).unwrap();

    let composition = match args[1].rsplit('.').next().unwrap_or_default() {
        "musicxml" => amm_sdk::storage::Storage::MusicXML.load_data(&content).unwrap(),
        "mid" | "smf" => amm_sdk::storage::Storage::MIDI.load_data(&content).unwrap(),
        _ => amm_sdk::storage::Storage::AMM.load_data(&content).unwrap(),
    };

    println!("{}", amm_sdk_netsblox::translate(&composition).unwrap());
}
