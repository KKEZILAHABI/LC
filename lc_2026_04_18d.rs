//Roman to Hindu-Arabic numeral converter
//  impl Solution {
//     pub fn roman_to_int(s: String) -> i32 {

//         let mut val: i32 = 0;

//         for (index, ch) in s.chars().rev().enumerate(){

//             if index == 0{
//                 val = Self::assigner(&ch);
//             }else {
//               if  let Some(prev) = s.chars().rev().nth(index - 1){
//                 if Self::assigner(&ch) < Self::assigner(&prev){
//                     val -= Self::assigner(&ch);
//                 } else{
//                     val += Self::assigner(&ch);
//                     } 
//                 }
//             }
//         }
        
//         return val
//     } 

//     pub fn assigner(ch: &char)-> i32{
//         match *ch{
//             'I'=> return 1,
//             'V' => return 5,
//             'X' => return 10,
//             'L' => return 50,
//             'C' => return 100,
//             'D' => return 500,
//             'M' => return 1000,
//               _ => return 0
//         }
//     }
// }

// Optimized
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut total = 0;
        let mut prev = 0;

        for ch in s.chars().rev() {
            let curr = Self::assigner(ch);

            if curr < prev {
                total -= curr;
            } else {
                total += curr;
            }

            prev = curr;
        }

        total
    }

    fn assigner(ch: char) -> i32 {
        match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }
}