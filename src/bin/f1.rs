use polars::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;

//use rand::Rng;
//use polars::df;

fn main() {
    let time_start = std::time::Instant::now();
    let mut v = vec![];
    let mut rng = rand::thread_rng();
    for i in 1..=5 {
        let rand_v: Vec<u32> = (0..8)
                        .map(|_| {let x = rng.gen_range::<u32,_>(0..=3); x} )
                        .collect();
        let name = format!("col{}", i);
        let s = Series::new(name.as_str(), rand_v.as_slice());
        v.push(s);
    }
    let mut some_df = DataFrame::new(v).expect("Failed create df");
    some_df = some_df.with_row_count("id").unwrap();
    println!("{:?}", some_df);
    
    
    let idx_cols: Vec<usize> = vec![1,2,4];
    let name_cols = some_df.get_column_names();
    let m = name_cols.iter().enumerate()
                    .filter(|t| idx_cols.contains(&t.0))
                    .map(|t| t.1)
                    .collect::<Vec<_>>();
    println!("{:?}", name_cols);
    println!("{:?}", m);
    let id_col = some_df.column("id").unwrap();

    let mut _idx_chunks = Vec::from(id_col.u32().unwrap())
        .into_iter()
        //.to_vec()
        //.len()
        //.into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    let me = _idx_chunks
        .chunks(3)
        .collect::<Vec<_>>()
        ;
    
    //println!("{:?}", some_df.select(["0","1"]).expect("colums not faund"));

    //println!("{:?}", some_df.select_at_idx(0).unwrap().sum::<i32>());
    for i in &me {
        //println!("{:?}", i);

        let _idx = Series::new("idx", i);
        let _idx = _idx.u32().unwrap();
        //let m:() = *idx;
        //let idx = UInt32Chunked::new("idx", &i);
        //println!("{:?}", some_df.take(&idx));
    }
    //let idx = UInt32Chunked::new("idx", &[1,2,22]);
    let r = some_df.select(["col1", "col2"]).unwrap();
    //let m: () = r;
    let mut dict: HashMap<_,_> = HashMap::new();
    r.column("col1").unwrap().u32().unwrap() .into_iter() .map(|x| x.unwrap())
        .zip(r.column("col2").unwrap()
                 .u32().unwrap()
                 .into_iter()
                 .map(|x| x.unwrap())
         )
        .for_each(|t| {
            dict.entry([t.0, t.1])
                .and_modify(|el| *el +=1 )
                .or_insert(1);
            }
        )
        ;
    
    dict = dict
            .into_iter()
            .filter(|t| t.1 >= 2 )
            .collect();
    
    println!("{:?}", dict);
    for (k,_) in dict {
        for idx in k {
            let mmm:() = idx;
            println!("idx_col = {:?}", idx)
        }
        //k.iter() .for_each(|el| println!("idx_col = {}", *el));

    }
    let mask = some_df.column("col1").unwrap().equal(2_i32) 
        & some_df.column("col2").unwrap().equal(1_i32);
    //let r = some_df.filter(&mask);
    //println!("{:?}", r);

    



    println!("{:?}", time_start.elapsed());
}
