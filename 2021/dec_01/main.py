import os
import pathlib

def readfile(file):
  return [int(x) for x in pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text().splitlines()]

def readinput():
  return readfile("input.in")

def solution1(depths):
  return calculate_solution(depths, 1)

def solution2(depths):
  return calculate_solution(depths, 3)

def calculate_solution(depths, measurements):
  count = 0
  for i in range(measurements, len(depths)):
    if depths[i] > depths[i-measurements]:
      count += 1

  return count

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {solution1(data)}")
  print(f"Solution 2: {solution2(data)}")