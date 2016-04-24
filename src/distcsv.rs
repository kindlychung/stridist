use csv::index::{Indexed, create_index};
use csv::Reader;
use std::io::{self, Write};
use std::fs;


trait ToVecF64 {
    fn to_vec_f64(self) -> Vec<f64>;
}

impl<'a> ToVecF64 for &'a [String] {
    fn to_vec_f64(self) -> Vec<f64> {
        self.into_iter()
            .map(|x| x.trim().parse::<f64>().expect("Invalid input, not a number."))
            .collect::<Vec<f64>>()
    }
}

fn get_index<F>(rdr: F) -> Indexed<fs::File, io::Cursor<Vec<u8>>>
    where F: Fn() -> Reader<fs::File>
{
    let mut index_data = io::Cursor::new(Vec::new());
    create_index(rdr(), index_data.by_ref()).unwrap();
    let index = Indexed::open(rdr(), index_data).unwrap();
    index
}


use ::Strategy;
use ::distfuncs::dist;
pub fn csv_dist<F>(rdr: F, strategy: &Strategy) -> (Vec<String>, Vec<Vec<f64>>)
    where F: Fn() -> Reader<fs::File>
{

    let mut index = get_index(rdr);
    let n_lines = index.count();
    index.seek(0).unwrap();
    let row = index.records().next().unwrap().unwrap();
    let n_cols = row.len();
    println!("✓ Read in {} records, each with {} STR counts.",
             n_lines,
             n_cols);
    let mut dist_mat: Vec<Vec<f64>> = Vec::with_capacity(n_lines as usize);
    let mut names: Vec<String> = Vec::with_capacity(n_lines as usize);

    println!("✓ Calculating distances...");
    'outer: for i in 0..n_lines {
        print!("\r✓ {}/{}", i+1, n_lines);
        let mut dist_vec: Vec<f64> = Vec::with_capacity((n_lines - i) as usize);
        index.seek(i).unwrap();
        let named_row = index.records().next().unwrap().unwrap();
        let (ref name, ref row1) = named_row.split_at(1);
        let row1 = row1.to_vec_f64();
        names.push(name[0].clone());
        'inner: for j in i..n_lines {
            if j == i {
                dist_vec.push(0f64);
                continue 'inner;
            }
            let _ = index.seek(j).unwrap();
            let named_row2 = index.records().next().unwrap().unwrap();
            let (_, row2) = named_row2.split_at(1);
            let row2 = row2.to_vec_f64();
            dist_vec.push(dist(&row1, &row2, strategy));
        }
        dist_mat.push(dist_vec);
    }
    println!("");
    let res = (names, dist_mat);
    res
}

#[test]
fn csv_dist_correct() {
    let f = ::test_file(&["xxx.csv"]);
    let rdr = || Reader::from_file(f.to_str().unwrap()).unwrap().has_headers(false);
    let (names, dist_mat) = csv_dist(rdr, &Strategy::Ads);
    assert_eq!(dist_mat.into_iter()
                       .map(|x| x.into_iter().map(|y| y as i64).collect::<Vec<_>>())
                       .collect::<Vec<_>>(),
               vec![vec![0i64, 5, 15], vec![0, 16], vec![0]]);
    assert_eq!(names, vec!["x1", "x2", "x3"]);
}

use std::string::ToString;
use std::io::BufWriter;
use std::fs::File;
pub fn write_csv<S: ToString>(names: &Vec<String>, mat: &Vec<Vec<S>>, outfile: &str) {
    let f = File::create(outfile).unwrap();
    let mut writer = BufWriter::new(f);
    // write the colnames, which are the same as the rownames. 
    let mut colnames = String::from(",");
    for n in names {
        colnames.push_str(n);
        colnames.push(',')
    }
    colnames.pop(); // the last comma should be removed. 
    colnames.push('\n'); // colnames finished
    let _ = writer.write(colnames.as_bytes());
    for (i, item) in mat.iter().enumerate() {
        // write the row name
        let _ = writer.write(names[i].as_bytes());
        let _ = writer.write(b",");
        // the distance matrix is symmetric, the first i cells of the ith row are not needed.
        let nan = (0..i).map(|_| ",").collect::<String>();
        let _ = writer.write(nan.as_bytes());
        for (j, item_inner) in item.iter().enumerate() {
            let _ = writer.write(item_inner.to_string().as_bytes());
            if j == (item.len() - 1) {
                let _ = writer.write(b"\n");
            } else {
                let _ = writer.write(b",");
            }
        }
    }
}
