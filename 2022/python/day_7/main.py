import re
import os


def part1():
    with open('./input.txt') as f:
        data = f.read().strip().split('\n')

    path = []
    files = {}
    dirs = {}

    total = 0

    for line in data:
        line = line.split(' ')

        if line[0] == '$':

            if line[1] == 'cd':
                if line[2] != '..':
                    path.append(line[2])
                else:
                    path = path[:-1]

        elif re.match('[0-9]+', line[0]):
            line[0] = int(line[0])
            total += line[0]
            file = os.path.join(*path, line[1])
            print(f'found file {line[1]} in {os.path.join(*path)}')
            for index in range(1, len(path) + 1):
                pwd = os.path.join(*path[:index])
                if pwd in dirs:
                    print(f'adding {line[0]} to {pwd} \n')
                    dirs[pwd] += line[0]
                    files[file] = line[0]
                else:
                    print(f'adding {line[0]} to {pwd} \n')
                    dirs[pwd] = line[0]
                    files[file] = line[0]
        else:
            pass

    ans = 0
    for k, v in dirs.items():
        if int(v) <= 100000:
            ans += v

    print(ans)


if __name__ == '__main__':

    with open('./input.txt') as f:
        data = f.read().strip().split('\n')

    path = []
    files = {}
    dirs = {}

    total = 0

    for line in data:
        line = line.split(' ')

        if line[0] == '$':

            if line[1] == 'cd':
                if line[2] != '..':
                    path.append(line[2])
                else:
                    path = path[:-1]

        elif re.match('[0-9]+', line[0]):
            line[0] = int(line[0])
            total += line[0]
            file = os.path.join(*path, line[1])
            print(f'found file {line[1]} in {os.path.join(*path)}')
            for index in range(1, len(path) + 1):
                pwd = os.path.join(*path[:index])
                if pwd in dirs:
                    print(f'adding {line[0]} to {pwd} \n')
                    dirs[pwd] += line[0]
                    files[file] = line[0]
                else:
                    print(f'adding {line[0]} to {pwd} \n')
                    dirs[pwd] = line[0]
                    files[file] = line[0]
        else:
            pass

    tot_file_space = 70000000
    needed_space = 30000000

    curr_free_space = tot_file_space - total
    space_to_find = needed_space - curr_free_space

    print(f'current free space is {curr_free_space}')
    print(f'need to find {space_to_find}')

    best = '/'
    for k, v in dirs.items():
        print(k, v)
        if v > space_to_find:
            if v < dirs[best]:
                best = k

    print(f'closest dir to space needed is {best} at {dirs[best]}')
