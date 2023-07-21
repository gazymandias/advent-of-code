def main():
    lines = open("input.txt", "r").read().split('\n\n')
    crates, instructions = [section.split("\n") for section in lines]
    number_of_crates = int(max(crates[-1:][0]))
    # clean
    crates = [crate.replace("    ", "[¬]").replace(' ', '').replace('[', '').replace(']', '') for crate in crates[:-1]]

    import collections

    # create crates
    stack = collections.OrderedDict()
    for i in range(0, int(number_of_crates), 1):
        stack['crate_%s' % (i + 1)] = []

    # add crate details to crates
    for list in crates:
        count = 1
        for char in list:
            stack[f'crate_{count}'].append(char)
            count += 1

    # reverse input and replace nulls with the weird symbol
    for i in range(0, number_of_crates, 1):
        try:
            # stack['crate_%s' % (i+1)]
            stack[f'crate_{i + 1}']
            # .reverse()
            stack[f'crate_{i + 1}'] = [i for i in stack[f'crate_{i + 1}'] if i != '¬']
        except Exception as e:
            continue

    # print(stack)
    move_ops = 0
    for instruction in instructions:
        instruction_split = instruction.split(' ')
        if len(instruction_split) > 1:
            _, number_crates, _, move_from, _, move_to = instruction_split
            to_move = stack['crate_%s' % (move_from)][:int(number_crates)]
            to_move.reverse()
            [stack['crate_%s' % (move_from)].remove(x) for x in to_move]
            # comment out for q2
            # to_move.reverse()
            [stack['crate_%s' % (move_to)].insert(0, x) for x in to_move]
            move_ops += 1
        else:
            continue

    print(stack)
    print(move_ops)

    for i in range(0, number_of_crates, 1):
        print(stack['crate_%s' % (i + 1)][:1][0])


if __name__ == '__main__':
    main()
