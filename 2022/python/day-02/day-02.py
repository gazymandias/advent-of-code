# part 1
def main():
    choices = {
        **dict.fromkeys(['A', 'X'], {'name': 'rock', 'score': 1, 'beats': 'scissors'}),
        **dict.fromkeys(['B', 'Y'], {'name': 'paper', 'score': 2, 'beats': 'rock'}),
        **dict.fromkeys(['C', 'Z'], {'name': 'scissors', 'score': 3, 'beats': 'paper'})
    }

    total_score = 0
    for line in open("input.txt"):
        score = 0
        opp, you = choices.get(line.rstrip()[0]), choices.get(line.rstrip()[2])
        # print(f"You choose {you['name']}, opponent chooses {opp['name']}")
        score += (6 if you["beats"] == opp["name"] else 3 if you["name"] == opp["name"] else 0)
        # print("You win" if score == 6 else "Draw" if score == 3 else "You lose")
        total_score += you["score"] + score
    print(total_score)

    # part 2

    total_score = 0
    for line in open("input.txt"):
        d = {'rock': 1, 'paper': 2, 'scissors': 3}
        score = 0
        opp_choice, required_outcome = line.rstrip()[0], line.rstrip()[2]
        opp = choices.get(line[0])
        # lose
        if required_outcome == 'X':
            you = opp["beats"]
        # draw
        elif required_outcome == 'Y':
            you = opp["name"]
        # win
        else:
            you = [x for x in filter(lambda name: name not in (opp["name"], opp["beats"]), [key for key in d.keys()])][
                0]
        print(f"You choose {you}, opponent chooses {opp['name']} (beats {opp['beats']})")
        score += (0 if you == opp["beats"] else 3 if you == opp["name"] else 6)
        total_score += d[you] + score
    print(total_score)


if __name__ == '__main__':
    main()
