use t1::{some_funcs};

fn main() {
    let start_time = std::time::Instant::now();
    let mut df_data = some_funcs::create_rand_df(43, 20);
    df_data = df_data.with_row_count("id").unwrap();
    some_funcs::preprocess_data(&mut df_data);
    // отримання списку всіх індексів
    let v = some_funcs::get_idx_cols(1, 8);

    let mut start: usize = 0;
    const BUNCH: usize = 10;
    const TARGET_SIZE: usize = 5;

    while start+BUNCH < df_data.height() {
        let end_db = if start+BUNCH >= df_data.height() { 
            df_data.height()
        } else { 
            start+BUNCH 
        };
        let end_tb = if start+BUNCH+TARGET_SIZE >= df_data.height() { 
            df_data.height()
        } else { 
            start+BUNCH+TARGET_SIZE 
        };
        let data_bunch = df_data.take_iter(start..end_db).unwrap();
        let target_bunch = df_data.take_iter(start+BUNCH..end_tb).unwrap();
        start += TARGET_SIZE;
        
        for row in target_bunch.iter_chunks() {
            for cols in &v {
                let all_cols = df_data.get_column_names();
                let choosen_cols = all_cols.iter().enumerate()
                    .filter(|t| cols.contains(&(t.0 as u8)))
                    .map(|t| t.1)
                    .collect::<Vec<_>>();
            }
            let mmm:() = row;
            println!("shit");
            println!("{:?}", row);
        }
        break;
    }


    println!("{:?}", start_time.elapsed());
}
