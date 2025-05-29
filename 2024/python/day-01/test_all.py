from typing import List

from part1 import total_distance, total_similarity

list_1: List[int] = [3,4,2,1,3,3]
list_2: List[int] = [4,3,5,3,9,3]

def test_part1():
    assert total_distance(list_1, list_2) == 11

def test_part2():
    assert total_similarity(list_1, list_2) == 31