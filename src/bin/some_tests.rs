//use std::collections::HashMap;
//use rand::prelude::*;
use polars::prelude::*;
use t1::{some_funcs};

fn main() {
    let mut df_data = some_funcs::create_rand_df(500, 20);
    df_data = df_data.with_row_count("id").unwrap();
    println!("{:?}", df_data);
    let rows = df_data.iter_chunks().next().unwrap();
    
    let start_time = std::time::Instant::now();
    for idx in 0..df_data.height() {
        dbg!(df_data.slice(idx as i64,1));
        //let row_index = UInt32Chunked::new("_", &[idx as u32]);
        //dbg!(df_data.take(&row_index).unwrap());
    }
    
    println!("{:?}", start_time.elapsed());
    dbg!(df_data);
}
