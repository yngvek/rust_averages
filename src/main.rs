//Given a list of integers, use a vector and return:
// mean (the average value), 
//median (when sorted, the value in the middle position), 
//mode (the value that occurs most often; 
//a hash map will be helpful here) of the list.
//testmer
use std::collections::HashMap;

fn main() {
    let mut list = vec![8,11,44,14,40,30,14,14];
    list.sort();
    println!("Mean is {}", mean(&list));
    println!("Median is {}", median(&list));
    println!("Mode is {}", mode(&list));
      
    fn mean(list: &[i32]) -> f64 {
        let sum: i32 = list.iter().sum();
        f64::from(sum) / (list.len() as f64)
     }

    fn median (list: &[i32]) -> f64 {
        let len = list.len();
        let mid = len / 2;
         if len % 2 == 0 {
           mean(&list[(mid - 1)..(mid + 1)])
          //alternativt:
          //let sum: i32 = list[(mid -1)..(mid +1)].iter().sum();
          //f64::from(sum) / (mid as f64)
         }
         else {
            f64::from(list[mid])
           }
     }

     fn mode (list: &[i32]) -> &i32 {
        let mut map = HashMap::new();
        for num in list {
          let count = map.entry(num).or_insert(0);
          *count += 1;
       }
       //println!("{:?}", map );
       let mut count_map: Vec<_> = map.iter().collect();
       count_map.sort_by(|a, b| b.1.cmp(a.1));
       //count_map[0].1
       //println!("{:?}", count_map);
       //println!("{}", count_map[0].0);
       count_map[0].0
     }
}


