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
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[0];
    //num referansı kullanımdayken v değiştirilemez
    v.push(4);
    println!("{:?}", *num); // Çıktı: [1, 2, 3, 4]
}