extern crate csv;
extern crate stridist;


macro_rules! five_times {
    ($x:expr) => (5 * $x);
}

macro_rules! myvec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}


macro_rules! let_ident {
    ($v:ident) => (let $v = 3);
}

macro_rules! def_func {
    () => [
        fn return333() -> u32 {
            333
        }
    ]
}


fn main() {
    let rdr = || csv::Reader::from_file("/tmp/xxx.csv").unwrap().has_headers(false);

    let mut index_data = io::Cursor::new(Vec::new());
    create_index(rdr(), index_data.by_ref()).unwrap();
    let mut index = Indexed::open(rdr(), index_data).unwrap();

    index.seek(0).unwrap();
    let row = index.records().next().unwrap().unwrap();
}
