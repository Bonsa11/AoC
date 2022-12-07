# i'd prefer to have a slow generic parser than hard code the 9 crates


def crate_parser(crates):
    socrocrates = crates.replace('[',' ').replace(']',' ').split('\n')
    n = int(socrocrates[-1].replace('  ','').replace(' ',',')[1:-1].split(',')[-1])
    values_to_pull = [1] + [2+4*x for x in range(1,n)]
    
    for val in values_to_pull:

        

    

if __name__ == '__main__':
    with open('./input.txt') as f:
        crates, action = f.read().split('\n\n')

    print(crate_parser(crates))