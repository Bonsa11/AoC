from typing import List, Tuple

def total_distance(list_1: List[int], list_2:List[int]) -> int:
    assert len(list_1) == len(list_2)
    total_distance = 0

    sorted_list_1 = sorted(list_1)
    sorted_list_2 = sorted(list_2)

    for a,b in zip(sorted_list_1, sorted_list_2):
        total_distance += abs(a-b)

    return total_distance


def total_similarity(list_1: List[int], list_2:List[int]) -> int:
    assert len(list_1) == len(list_2)
    similarity = 0

    for num in list_1:
        similarity += num * len([f for f in list_2 if f == num])

    return similarity



def _parse_input() -> Tuple[List[int], List[int]]:
    with open("2024/python/day-01/example.txt", "r") as f:
        data: List[str] = f.readlines()

    list1 = []
    list2 = []
    for nums in data:
        nums = [int(num) for num in nums.split("   ")]
        list1.append(nums[0])
        list2.append(nums[1])

    return (list1, list2)

if __name__ == "__main__":
    list1, list2 = _parse_input()
    distance = total_distance(list1, list2)
    print(f"Total distance is {distance}")
    similarity = total_similarity(list1, list2)
    print(f"Total similarity is {similarity}")
