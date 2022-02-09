use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm_kit::{execute_entry_point, ext, oei, prepare_entry_point};
use std::collections::HashMap;
const TOP_N_COINS: usize=10;
#[derive(OBIDecode, OBISchema)]
struct Input {
    multiplier: u64,
}

#[derive(OBIEncode, OBISchema)]
struct Result {
    names:Vec<String>,
    market_cap:Vec<u64>,
    rates: Vec<u64>,
}

const DATA_SOURCE_ID: i64 = 253;
const EXTERNAL_ID: i64 = 0;

fn median_int(arr: &mut Vec<u64>) -> u64 {
    let len_arr = arr.len() as u64;
    if len_arr > 0u64 {
      arr.sort();
      let mid = len_arr / 2;
      if len_arr as u64 % 2==0{
        (arr[(mid - 1u64) as usize]+arr[mid as usize]) / 2u64
      } else {
        arr[mid as usize]
      }
    }else{
      0u64
    }
  }

fn median_float(arr: &mut Vec<f64>) -> f64 {
  let len_arr = arr.len() as f64;
  if len_arr > 0f64 {
    arr.sort_by(|a,b| a.partial_cmp(b).unwrap());
    let mid = len_arr / 2f64;
    if len_arr as u64 % 2==0{
      (arr[(mid - 1f64) as usize]+arr[mid as usize]) / 2f64
    } else {
      arr[mid as usize]
    }
  }else{
    0f64
  }
}
#[no_mangle]
fn prepare_impl(_input: Input) {
    oei::ask_external_data(
        EXTERNAL_ID,
        DATA_SOURCE_ID,
        "empty".as_bytes(),
    );
}

#[no_mangle]
fn execute_impl(_input: Input) -> Result {
     let mut _exchange_medians:Option<Vec<f64>> = Some(vec![]);
     let raw_input = ext::load_input::<String>(EXTERNAL_ID);
     let mut maj_names: Vec<String>=Vec::new(); 
     let mut med_market_cap: Vec<u64>=Vec::new();
     let inputs:Vec<String> = raw_input.collect();
     let mut names_dict = HashMap::new();
     let mut names_vec=Vec::new();
     let mut total=0;
     if inputs.is_empty(){
       _exchange_medians = None;
     } else {
        let mut names: Vec<Vec<String>> = vec![vec![]; TOP_N_COINS]; 
        let mut market_cap: Vec<Vec<u64>> = vec![vec![]; TOP_N_COINS];
        let mut prices: Vec<Vec<f64>> = vec![vec![]; TOP_N_COINS];
       for raw in &inputs{
            let validator_price_list:Vec<&str> = raw
            .split(',')
            .collect();
            let mut all_strings = "".to_owned();
            for (index,&price) in validator_price_list.iter().enumerate() {
                if index<10{
                    names[index].push(price.to_owned());
                    all_strings.push_str(&price.to_owned());
                } else{
                    break;
                }
            }
            names_vec.push(all_strings.to_owned());
            let count = names_dict.entry(all_strings).or_insert(0);
            *count += 1;
            total +=1;
       }
       let mut result=String::new();
       for (key,value) in names_dict.into_iter() {
           if total%2==0&&value>=(total/2) {
               result=key.to_owned();
           }
           if total%2==1&&value>(total/2) {
                result=key.to_owned();
            }
       }
       let mut i=0;
       let mut stored_names_flag=0;
       for raw in inputs{
            if !result.eq(&names_vec[i]){
                i+=1;
                continue;
            }
            let validator_price_list:Vec<&str> = raw
            .split(',')
            .collect();
            for (index,&price) in validator_price_list.iter().enumerate() { 
                if index>=10{
                    if index<20{
                        market_cap[index-10].push(price.parse::<u64>().unwrap());
                    } else if index<30{
                        prices[index-20].push(price.parse::<f64>().unwrap());
                    }
                }else if stored_names_flag==0{
                    maj_names.push(price.to_owned());
                }
            }
            if stored_names_flag==0{
                stored_names_flag=1;
            }
      }
      let mut med_prices: Vec<f64>=Vec::new();
       for (_,price) in prices.iter().enumerate(){
         med_prices.push(median_float(&mut price.to_vec()));
       }
       for (_,market) in market_cap.iter().enumerate(){
        med_market_cap.push(median_int(&mut market.to_vec()));
      }
       _exchange_medians = Some(med_prices);
     }
     let mut rates:Vec<u64> = Vec::new();
     if _exchange_medians.is_some() {
       let exchange_medians = _exchange_medians.unwrap();
       for item in &exchange_medians {
         rates.push(((*item)*(_input.multiplier as f64)) as u64);
       }
     }
     Result {names:maj_names, market_cap:med_market_cap,rates: rates }
}

prepare_entry_point!(prepare_impl);
execute_entry_point!(execute_impl);

