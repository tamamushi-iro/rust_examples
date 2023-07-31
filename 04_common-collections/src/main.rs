use std::collections::HashMap;

fn median(v: &mut Vec<i32>) -> i32 {
    v.sort();
    v[(v.len()/2) - 1]
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut hm = HashMap::new();
    for i in v {
        let count = hm.entry(i).or_insert(0);
        *count += 1;
    }
    let mut max_k = -9999;
    let mut max_v = -9999;
    for (k, v) in hm {
        // println!("{k}: {v}");
        if max_v < v {
            max_v = v;
            max_k = *k;
        }
    }
    max_k
}

// TODO
// fn to_pig_latin(s: &str) -> String {
//     let s = String::from(s);
//     for word in s.split_whitespace() {
//         if &word[0..1] {
//             print(word)
//         }
//     }
//     s
// }

fn main() {
    // 1: Vectors
    let mut v = vec![8, 1, 2, 45, 45, 45, 45, 456, 2, 1, 9, 9, 0, 1, 8];
    let median = median(&mut v);
    println!("Median: {median}");
    let mode = mode(&v);
    println!("Mode: {mode}");

    // // 2: Strings
    // let s = "I'll have you know I graduated top of my class in the Navy Seals, and I've been involved in numerous secret raids on Al-Quaeda, and I have over 300 confirmed kills.";
    // println!("String: {s}\nPig-latin: {}", to_pig_latin(&s));
}
