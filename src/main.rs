fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("usage: {} [input path]", args[0]);
    }
    let input = std::fs::read(&args[1]).unwrap();
    let composition = amm_sdk::storage::Storage::AMM.load_data(&input).unwrap();
    println!("{}", amm_sdk_netsblox::translate(&composition).unwrap());
}
