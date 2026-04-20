//Furthest Different Colored houses
// impl Solution {
//     pub fn max_distance(colors: Vec<i32>) -> i32 {
//         let mut distances: Vec<i32> = Vec::new();
//         for i in 0..colors.len(){
//             for j in (0..colors.len()).rev(){
//                 if colors[i] != colors[j]{
//                     distances.push(i.abs_diff(j) as i32);
//                 }
//             }
//         }
//         if let Some(max_dist) = distances.iter().max(){
//             *max_dist
//         }else{
//             0
//         }
//     }
// }
//Optmized
impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        
        let mut max_dist = 0;

        // Compare with first element
        for j in (0..n).rev() {
            if colors[j] != colors[0] {
                max_dist = j as i32;
                break;
            }
        }

        // Compare with last element
        for i in 0..n {
            if colors[i] != colors[n - 1] {
                max_dist = max_dist.max((n - 1 - i) as i32);
                break;
            }
        }

        max_dist
    }
}