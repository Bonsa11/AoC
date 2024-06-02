# a = rock
# b = paper
# c = scissors

# x = rock [1]
# y = paper [2]
# z = scissors [3]

# 0 for loss
# 3 for draw
# 6 for win


def part1():
    with open('./input.txt') as f:
        data = f.read()
        data = data.replace('X', 'A').replace('Y', 'B').replace('Z', 'C')
        data = data.split('\n')
        data = [d.split() for d in data]

    print(data[:5])

    played = {'A': 1,
              'B': 2,
              'C': 3}

    wins = [['A', 'B'], ['B', 'C'], ['C', 'A']]

    pts = 0
    for match in data:
        pts += played[match[1]]
        if match[0] == match[1]:
            pts += 3
        elif match in wins:
            pts += 6

    return pts


if __name__ == '__main__':
    with open('./input.txt') as f:
        data = f.read()
        data = data.replace('X', 'A').replace('Y', 'B').replace('Z', 'C')
        data = data.split('\n')
        #data = [d.split() for d in data]

    print(data[:5])

    played = {'A A': 3,
              'A B': 1,
              'A C': 2,
              'B A': 1,
              'B B': 2,
              'B C': 3,
              'C A': 2,
              'C B': 3,
              'C C': 1
            }

    result = {'A': 0,
              'B': 3,
              'C': 6}

    wins = [['A', 'B'], ['B', 'C'], ['C', 'A']]

    pts = 0
    for match in data:
        pts += result[match[2]]
        pts += played[match]

    print(pts)
