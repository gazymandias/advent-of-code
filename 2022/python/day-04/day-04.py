def main():
    lines = open("input.txt", "r").read().splitlines()
    matching_pairs_q1 = 0
    matching_pairs_q2 = 0
    for line in lines:
        elf_1, elf_2 = line.split(sep=',')
        elf_1_range = range(int(elf_1.split('-')[0]), int(elf_1.split('-')[1]) + 1)
        elf_2_range = range(int(elf_2.split('-')[0]), int(elf_2.split('-')[1]) + 1)
        is_matching = set(elf_1_range).issubset(elf_2_range) or set(elf_2_range).issubset(elf_1_range)
        is_matching_q2 = (set(elf_1_range) & set(elf_2_range))
        matching_pairs_q1 += 1 if is_matching else 0
        matching_pairs_q2 += 1 if is_matching_q2 else 0
    print(matching_pairs_q1)
    print(matching_pairs_q2)


if __name__ == '__main__':
    main()
