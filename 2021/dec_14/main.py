import os
import pathlib

def readfile(file):
  lines = pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text().splitlines()
  template = lines[0]
  rules = {}
  for line in [x.split(" -> ") for x in lines[2:]]:
    rules[line[0]] = line[1]

  return (template, rules)

def readinput():
  return readfile("input.in")

def find_nr_occurences(start, end, rules, occurences, known_occurences, level, max_level):
  idx = (start, end, level)

  if level == max_level:
    known_occurences[idx] = {}
    return occurences, known_occurences

  if idx in known_occurences.keys():
    for key in known_occurences[idx].keys():
      occurences[key] += known_occurences[idx][key]
    return occurences, known_occurences

  new_char = rules[start + end]
  occurences[new_char] += 1

  occurences, known_occurences = find_nr_occurences(start, new_char, rules, occurences, known_occurences, level+1, max_level)
  occurences, known_occurences = find_nr_occurences(new_char, end, rules, occurences, known_occurences, level+1, max_level)

  # Update known occurences
  known_occurences[idx] = {}
  for key in occurences.keys():
    known_occurences[idx][key] = 0
  for key in known_occurences[(start, new_char, level+1)].keys():
    known_occurences[idx][key] += known_occurences[(start, new_char, level+1)][key]
  for key in known_occurences[(new_char, end, level+1)].keys():
      known_occurences[idx][key] += known_occurences[(new_char, end, level+1)][key]
  known_occurences[idx][new_char] += 1

  return occurences, known_occurences

def solution(template, rules, nr_steps):
  # Create dict for all characters
  nr_occurences = {}
  for c in rules.values():
    if c not in nr_occurences.keys():
      nr_occurences[c] = 0

  known_occurences = {}
  for i in range(len(template)-1):
    nr_occurences[template[i]] += 1
    nr_occurences, known_occurences = find_nr_occurences(template[i], template[i+1], rules, nr_occurences, known_occurences, 0, nr_steps)

  nr_occurences[template[-1]] += 1

  return max(nr_occurences.values()) - min(nr_occurences.values())

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {solution(*data, 10)}")
  print(f"Solution 2: {solution(*data, 40)}")