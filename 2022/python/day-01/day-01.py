def main():
    elves = []
    count = 0
    total_calories = 0

    for line in open("input.txt"):
        print(line)
        if not line.strip():
            elves.append(total_calories)
            count += 1
            total_calories = 0
        else:
            total_calories += int(line.rstrip())
    elves_ranked_by_calories = sorted(elves, reverse=True)
    print(elves_ranked_by_calories[0])
    print(sum(elves_ranked_by_calories[:3]))


if __name__ == '__main__':
    main()
