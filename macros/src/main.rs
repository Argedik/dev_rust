// macro_rules! check_even {
//     ($num:expr) => {
//         if $num % 2 == 0 {
//             println!("{} is even.", $num);
//         } else {
//             println!("{} is odd.", $num);
//         }
//     };
// }

// macro_rules! my_vec {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }

fn main() {
    // check_even!(10); // Çıktı: 10 is even.
    // check_even!(7);  // Çıktı: 7 is odd.
    // let v = my_vec![1, 2, 3, 4];
    // let mut v: Vec<i32> = vec![1, 2, 3];
    // let num: &i32 = &v[0];
    //num referansı kullanımdayken v değiştirilemez
    // println!("{:?}", num); // Çıktı: [1, 2, 3, 4]

    // return_a_string();
}