def main():
    string = open("input.txt", "r").read()
    characters = 0
    d = {
        'packets': 4,
        'messages': 14
    }
    for key, value in d.items():
        while True:
            check_string = string[characters:characters + value]
            if len(set(check_string)) == value:
                print(f"{key}: {characters + value}")
                break
            else:
                characters += 1


if __name__ == '__main__':
    main()
