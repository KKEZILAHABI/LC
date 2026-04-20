#Furthest Different Colored houses
class Solution(object):
    def maxDistance(self, colors):
        """
        :type colors: List[int]
        :rtype: int
        """
        distances = []
        colors2 = colors.reverse()
        for i in range(len(colors)):
            for j in range(len(colors)):
                if colors[i] != colors[j]:
                    distances.append(abs(i-j))
        return max(distances)

        