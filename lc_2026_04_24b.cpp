// Furthest Dist From Origin
class Solution {
public:
    int furthestDistanceFromOrigin(string moves) {

    int left = 0;
    int neut = 0;
    int right = 0;

    for( int i = 0; i< moves.length(); i++){
        if (moves[i] == 'L'){
            left++;
        }
        else if (moves[i] == 'R'){
            right++;
        }
        else if (moves[i] == '_'){
            neut++;
        }
    }

    if (left > right){
        return (left + neut - right);
    }
    else{
        return (right + neut - left);
    } 
    }
};
