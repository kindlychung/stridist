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
    let mut rdr = csv::Reader::from_file("/tmp/xxx.csv").unwrap();
    for record in rdr.decode() {
        let (name, i1, i2, i3): (String, u32, u32, u32) = record.unwrap();
        println!("{}: {}, {}, {}", name, i1, i2, i3);
    }
    let x: Vec<i32> = myvec!(1, 2, 3);
    println!("{:?}", x);
    println!("{}", five_times!(2 + 6));
    let_ident!(xx);
    println!("{}", xx);
    def_func!();
    println!("{}", return333());
    use distfuncs::addtwo;
    println!("{:?}", addtwo(32));
}
