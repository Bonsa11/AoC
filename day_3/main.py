def part1():
    with open('./input.txt') as f:
        data = f.read().split('\n')

    priorities = []
    for rs in data:
        L = len(rs)
        rs = [ord(x) - 96 if x.islower() else ord(x) - 38 for x in rs]
        comp1 = rs[0: L // 2]
        comp2 = rs[L // 2:]
        priorities.append(list(set(comp1).intersection(comp2))[0])

    print(sum(priorities))


if __name__ == '__main__':
    with open('./input.txt') as f:
        data = f.read().split('\n')

    priorities = []
    rp = 3
    while rp <= len(data):
        bags = data[rp-3:rp]
        print(bags)
        for index, bag in enumerate(bags):
            bags[index] = set([ord(x) - 96 if x.islower() else ord(x) - 38 for x in bag])
        val = list(set.intersection(*bags))[0]
        priorities.append(val)
        rp += 3

    print(sum(priorities))


