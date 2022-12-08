import numpy as np


def check_left(data, i, j):
    return data[i,j] <= max(data[i,:j])

def check_right(data, i, j):
    return data[i,j] <= max(data[i,j+1:])

def check_up(data, i, j):
    return data[i,j] <= max(data[:i,j])

def check_down(data, i, j):
    return data[i,j] <= max(data[i+1:,j])

def check_dirs(data, i, j):
    if check_left(data, i, j) and check_right(data, i, j) and check_up(data, i, j) and check_down(data, i, j):
        return True
    else:
        return False
        
def part1():

    with open('./input.txt') as f:
        data = f.read().strip().split('\n')

        # get list of lists
        for index, row in enumerate(data):
            data[index] = [ch for ch in row]

        data = np.array(data)

        count = 0

        #add edge trees
        count += 2*data.shape[0]
        count += 2*(data.shape[1]-2)

        print(count)

        # check internal trees
        for i in range(1,data.shape[0]-1):
            for j in range(1,data.shape[1]-1):
                if not check_dirs(data, i, j):
                    print(i,j)
                    count += 1

        print(count)
                
def check_left_view(data, i, j):
    score = 0
    our_height = data[i,j]
    for height in data[i,:j][::-1]:
        if our_height > height:
            score += 1
        else:
            score += 1
            break

    return score

def check_right_view(data, i, j):
    score = 0
    our_height = data[i,j]
    for height in data[i,j+1:]:
        if our_height > height:
            score += 1
        else:
            score += 1
            break

    return score

def check_up_view(data, i, j):
    score = 0
    our_height = data[i,j]
    for height in data[:i,j][::-1]:
        if our_height > height:
            score += 1
        else:
            score += 1
            break

    return score

def check_down_view(data, i, j):
    score = 0
    our_height = data[i,j]
    for height in data[i+1:,j]:
        if our_height > height:
            score += 1
        else:
            score += 1
            break

    return score

def check_view(data, i, j):
    score = 1
    score *= check_left_view(data, i, j)
    score *= check_right_view(data, i, j)
    score *= check_up_view(data, i, j)
    score *= check_down_view(data, i, j)

    return score

if __name__ == '__main__':

    with open('./input.txt') as f:
        data = f.read().strip().split('\n')

        # get list of lists
        for index, row in enumerate(data):
            data[index] = [ch for ch in row]

        data = np.array(data)
        
        score = 1
        # check internal trees
        for i in range(1,data.shape[0]-1):
            for j in range(1,data.shape[1]-1):
                new_score = check_view(data,i,j)
                if new_score > score:
                    score = new_score

    print(score)