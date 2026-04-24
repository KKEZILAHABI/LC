
#Furthest Dist From Origin
class Solution(object):
    def furthestDistanceFromOrigin(self, moves):
        """
        :type moves: str
        :rtype: int
        """
        moves = list(moves)
        left, right, neut = 0,0,0

        for ch in moves:
            if ch == 'R':
                right += 1
            elif ch == 'L':
                left += 1
            elif ch == '_':
                neut += 1

        if left > right:
            return left + neut - right
        else:
            return right + neut - left
