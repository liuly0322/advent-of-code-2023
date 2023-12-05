input_data = open("input.txt", "r").read()

# part2 need data preprocessing
replace_dict = {
    "one": "one1one",
    "two": "two2two",
    "three": "three3three",
    "four": "four4four",
    "five": "five5five",
    "six": "six6six",
    "seven": "seven7seven",
    "eight": "eight8eight",
    "nine": "nine9nine",
}

IS_PART_TWO = True

if IS_PART_TWO:
    for replace_item in replace_dict.items():
        input_data = input_data.replace(replace_item[0], replace_item[1])

input_data = input_data.split("\n")
filter_first = lambda s: [c for c in s if c.isdigit()][0]
filter_last = lambda s: [c for c in s if c.isdigit()][-1]
input_data = [int(filter_first(s) + filter_last(s)) for s in input_data]
print(sum(input_data))
