

def part1():
    with open('./input.txt') as f:
        data = f.read().split('\n')

    pairs = [x.split(',') for x in data]

    count = 0
    for pair in pairs:
        lhs, rhs = pair

        lhs_lower, lhs_higher = lhs.split('-')
        rhs_lower, rhs_higher = rhs.split('-')

        if int(lhs_lower) <= int(rhs_lower) and int(lhs_higher) >= int(rhs_higher):
            count += 1
        elif int(lhs_lower) >= int(rhs_lower) and int(lhs_higher) <= int(rhs_higher):
            count += 1

    print(count)

if __name__ == '__main__':
    with open('./input.txt') as f:
        data = f.read().split('\n')

    pairs = [x.split(',') for x in data]

    count = 0
    for pair in pairs:
        lhs, rhs = pair
        lhs_lower, lhs_higher = lhs.split('-')
        rhs_lower, rhs_higher = rhs.split('-')

        if not (int(lhs_lower) > int(rhs_higher) or int(lhs_higher) < int(rhs_lower)):
            count += 1

    print(count)
