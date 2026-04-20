# class Solution(object):
#     def maxDistance(self, colors):
#         """
#         :type colors: List[int]
#         :rtype: int
#         """
#         distances = []
#         colors2 = colors.reverse()
#         for i in range(len(colors)):
#             for j in range(len(colors)):
#                 if colors[i] != colors[j]:
#                     distances.append(abs(i-j))
#         return max(distances)

#Optimized
class Solution(object):
    def maxDistance(self, colors):
        n = len(colors)
        
        max_dist = 0
        
        # Compare with first element
        for j in range(n - 1, -1, -1):
            if colors[j] != colors[0]:
                max_dist = j
                break
        
        # Compare with last element
        for i in range(n):
            if colors[i] != colors[n - 1]:
                max_dist = max(max_dist, n - 1 - i)
                break
        
        return max_dist