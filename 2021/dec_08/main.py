import os
import pathlib

def readfile(file):
  return [(y[0].split(), y[1].split()) for y in [x.split(" | ") for x in pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text().splitlines()]]

def readinput():
  return readfile("input.in")

def solution1(data):
  count = 0
  for input, output in data:
    for segment in output:
      if len(segment) == 2 or len(segment) == 3 or len(segment) == 4 or len(segment) == 7:
        count += 1
  return count

def overlaps(left, right):
  set_left = set(left)
  set_right = set(right)

  return set_left.issubset(set_right) or set_right.issubset(set_left)

#  0000
# 1    2
# 1    2
#  3333
# 4    5
# 4    5
#  6666
def find_number_representations(input):
  nr_to_string = 10 * ['']

  # sort inputs
  sorted_inputs = [''.join(sorted(x)) for x in input]

  # unique paterns (1,4,7,8)
  for i in sorted_inputs:
    if len(i) == 2:
      nr_to_string[1] = i
    elif len(i) == 4:
      nr_to_string[4] = i
    elif len(i) == 3:
      nr_to_string[7] = i
    elif len(i) == 7:
      nr_to_string[8] = i
  
  # Paterns with length 6 (0, 6, 9)
  # Overlaps?
  #     1 | 4
  # 9 | x | y (x = dont care)
  # 0 | y | n
  # 6 | n | n
  for i in sorted_inputs:
    if len(i) == 6:
      if overlaps(nr_to_string[4], i):
        nr_to_string[9] = i
      elif not overlaps(nr_to_string[1], i):
        nr_to_string[6] = i
      elif overlaps(nr_to_string[1], i):
        nr_to_string[0] = i
      else:
        print(f"ERROR for {i}, current list: {sorted_inputs}")

  # Paterns with length 5 (2, 3, 5)
  # Overlaps?
  #     1 | .
  # 3 | y | x (x = dont care)
  # 2 | n | ? <- no overlaps, compare with 9 => 2 missing overlap
  # 5 | n | ? <- no overlaps, compare with 9 => 1 missing overlap
  for i in sorted_inputs:
    if len(i) == 5:
      if overlaps(nr_to_string[1], i):
        nr_to_string[3] = i
      elif len(set(nr_to_string[9]).difference(set(i))) == 2:
        nr_to_string[2] = i
      elif len(set(nr_to_string[9]).difference(set(i))) == 1:
        nr_to_string[5] = i
      else:
        print(f"ERROR for {i}, current list: {sorted_inputs}")

  return nr_to_string

def decode(input, output):
  nr_to_string = find_number_representations(input)
  sorted_output = [''.join(sorted(x)) for x in output]
  val = 0
  for digit in range(len(sorted_output)):
    val += nr_to_string.index(sorted_output[digit]) * pow(10, len(sorted_output)-digit-1) 
  return val

def solution2(data):
  return sum([decode(x[0],x[1]) for x in data])

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {solution1(data)}")
  print(f"Solution 2: {solution2(data)}")