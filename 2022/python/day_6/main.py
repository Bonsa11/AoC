
def part1():
    with open('./input.txt') as f:
        data = f.read()

    lp = 0
    while lp < len(data):
        # this feels like cheating
        if len(set(data[lp:lp +4])) == 4:
            break
        else:
            lp += 1

    print(lp +4)

if __name__ == '__main__':
    with open('./input.txt') as f:
        data = f.read()

    lp = 0
    while lp < len(data):
        if len(set(data[lp:lp + 14])) == 14:
            print(set(data[lp:lp + 14]))
            break
        else:
            lp += 1

    print(lp + 14)
