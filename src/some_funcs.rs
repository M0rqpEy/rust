use polars::prelude::*;
use rand::prelude::*;

fn my_rec(count_cols: u8, 
          start_range: u8,
          end_range: u8,
          v: &mut Vec<Vec<u8>>,
          middle_v:  &mut Vec<u8>,
          c: &mut u32) {

    for ind in start_range..=end_range-count_cols{
        if middle_v.len() > 0 && ind <= middle_v[middle_v.len()-1] {continue};
        middle_v.push(ind);       
        if count_cols != 1 { 
            my_rec(count_cols-1, start_range+1, end_range, v, middle_v, c) 
        } else {
            //println!("{:?}", middle_v);
            *c+=1;
            v.push(middle_v.to_vec());
            middle_v.pop();
        } 
    }
    middle_v.pop();
}
pub fn get_idx_cols(start_idx: u8, count_cols: u8) -> Vec<Vec<u8>>{
    let mut v = vec![vec![0u8;0];0];
    let mut c = 0_u32;
    for col in 1..=count_cols {
        my_rec(col, start_idx, start_idx+16, &mut v,&mut vec![0u8;0], &mut c);
    }
    v
}

pub fn create_rand_df(rows: u32, cols: u8) -> DataFrame {
    let mut v = vec![];
    let mut rng = rand::thread_rng();
    for col in 0..cols {
        let data: Vec<u32> = (0..rows)
                        .map(|_| {let r = rng.gen_range::<u32,_>(0..=3); r})
                        .collect();
        let name_col = format!("col{}", col); 
        let s = Series::new(name_col.as_str(), data.as_slice());
        v.push(s);
    }
    let df_data = DataFrame::new(v).expect("Failed create df");
    df_data
}

pub fn preprocess_data(df_data: &mut DataFrame) {
    let mut v = vec![];
    for _ in 0..4 {
        let last_s = df_data.pop();
       v.push(last_s.unwrap());
    }
    let home_sum = &v[3] + &v[2];
    let away_sum = &v[1] + &v[0];
    let r = home_sum.gt(&away_sum);
    let r_vec: Vec<u32> = r.into_iter()
                    .map(|x| x.unwrap() as u32)
                    .collect();
    let r_s = Series::new("rez", r_vec.as_slice());
    *df_data = df_data.hstack(&[r_s]).unwrap();
}
