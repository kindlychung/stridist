#![feature(plugin)]
#![plugin(docopt_macros)]
extern crate csv;
extern crate stridist;
extern crate rustc_serialize;
extern crate docopt;

use stridist::distcsv::*;
use stridist::Strategy;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

docopt!(Args derive Debug, "
Usage: 
    stridist <strfile> [--strategy <strategy>]  --out <outfile> 
    stridist (--help | --version)

Options:
    --help                   Show this message.
    --version                Show the version of rustc.
    --strategy <strategy>    Choose a strategy. At the moment there are two possibilities: Euclidean or Ads.
                             Default is Ads, it's the simplest. 
    --out <outfile>          Path to output file.
", flag_out: String, flag_strategy: Option<Strategy>, arg_strfile: String);


fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    if *&args.flag_version {
        println!("{}", VERSION);
        std::process::exit(0);
    }
    //println!("{:?}", args);
    let rdr = || csv::Reader::from_file(&args.arg_strfile).unwrap().has_headers(false);
    let (names, dist_mat) = csv_dist(rdr, &args.flag_strategy.unwrap_or(Strategy::Ads));
    write_csv(&names, &dist_mat, &args.flag_out);
}
