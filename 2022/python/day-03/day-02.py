def main():
    import string

    # task 1
    item_priorities_sum = 0
    lines = open("input.txt", "r").read().splitlines()
    for line in lines:
        trimmed_line = line.rstrip()
        split_point = int(len(trimmed_line)/2)
        comp_1, comp_2 = trimmed_line[:split_point], trimmed_line[split_point:]
        shared_item_type = list(set(comp_1)&set(comp_2))[0]
        numbers = list(range(1,27))+list(range(27,53))
        letters = string.ascii_lowercase + string.ascii_uppercase
        priorities = dict(zip(letters, numbers))
        item_priority = priorities.get(shared_item_type)
        item_priorities_sum += item_priority

    print(item_priorities_sum)

    # task 2
    item_priorities_sum = 0
    lines = open("input.txt", "r").read().splitlines()
    chunks = 3
    for i in range(0, len(lines), chunks):
        x = i
        group = lines[x:x+chunks]
        shared_item_type =  list(set(group[0])&set(group[1])&set(group[2]))[0]
        numbers = list(range(1,27))+list(range(27,53))
        letters = string.ascii_lowercase + string.ascii_uppercase
        priorities = dict(zip(letters, numbers))
        # priorities = string.ascii_letters
        item_priority = priorities.get(shared_item_type)
        item_priorities_sum += item_priority
    print(item_priorities_sum)


    # task 2
    item_priorities_sum = 0
    lines = open("input.txt", "r").read().splitlines()
    chunks = 3
    for i in range(0, len(lines), chunks):
        group = lines[i:i+chunks]
        print(group)
        shared_item_type =  list(set(group[0])&set(group[1])&set(group[2]))[0]
        item_priorities_sum += string.ascii_letters.index(shared_item_type)+1

    print(item_priorities_sum)


if __name__ == '__main__':
    main()