//Furthest Dist From Origin
impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {

        let mut left: i32 = 0;
        let mut right: i32 = 0;
        let mut neut: i32 = 0;

        for ch in moves.chars(){
            if ch == 'R'{
                right += 1;
            }
            else if ch == 'L'{
                left += 1;
            }
            else if ch == '_'{
                neut += 1;
            }
        }

        if left > right{
            left + neut - right
        }
        else{
            right + neut - left
        }
    }
}
