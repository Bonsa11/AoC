# i'd prefer to have a slow generic parser than hard code the 9 crates
import re

def crate_parser(crates):
    'move n rows of crates into a nice format'
    #this could al be replaced by parsing bottom row for where values != ' '
    socrocrates = crates.replace('[',' ').replace(']',' ').split('\n') 
    n = int(socrocrates[-1].replace('  ','').replace(' ',',')[1:-1].split(',')[-1])
    values_to_pull = ([1] + [1+4*x for x in range(1,n)])

    data = {}
    for count, val in enumerate(values_to_pull):
        vals = []
        for row in socrocrates[:-1]:
            if row[val] != ' ':
                vals.append(row[val])
        data[count+1] = vals[::-1]

    return data


def CrateMover9000(data, n, a, b):
    'move top n crates from a to b'
    for _ in range(n): 
        data[b] += data[a][-1]
        data[a] = data[a][:-1]
    return data

def CrateMover9001(data, n, a, b):
    'move top n crates from a to b' 
    data[b] += data[a][-n:]
    data[a] = data[a][:-n]
    return data

def parse_action(action):
    'get instructions from actions'
    nums = re.findall('\d+',action)
    return [int(x) for x in nums]
    
def part1():
    with open('./input.txt') as f:
        crates, actions = f.read().split('\n\n')

    stacks = crate_parser(crates)
    actions = actions.split('\n')

    for a,b in stacks.items():
        print(b)
    print('\n\n')

    for action in actions:
        n,a,b = parse_action(action)
        stacks = CrateMover9000(stacks, n, a, b)

        for a,b in stacks.items():
            print(b)
        print('\n\n')

    print(''.join(v[-1] for k,v in stacks.items()))


if __name__ == '__main__':
    with open('./input.txt') as f:
        crates, actions = f.read().split('\n\n')

    stacks = crate_parser(crates)
    actions = actions.split('\n')

    for a,b in stacks.items():
        print(b)
    print('\n\n')

    for action in actions:
        n,a,b = parse_action(action)
        stacks = CrateMover9001(stacks, n, a, b)

        for a,b in stacks.items():
            print(b)
        print('\n\n')

    print(''.join(v[-1] for k,v in stacks.items()))
    
        