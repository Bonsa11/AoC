
if __name__ == '__main__':
    with open('./input.txt') as f:
        data = f.read().split('\n')

    elfs = []
    lp = -1
    rp = 0
    while rp < len(data):
        if data[rp] != '':
            rp += 1
        else:
            elfs.append(sum(int(x) for x in data[lp+1:rp]))
            lp = rp
            rp += 1
    elfs.append(sum(int(x) for x in data[lp+1:rp]))


    # answers for part 1 and 2
    print(max(elfs))
    print(sum(sorted(elfs)[-3:]))