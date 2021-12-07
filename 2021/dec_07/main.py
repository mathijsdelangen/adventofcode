import os
import pathlib
from statistics import median

def readfile(file):
  return [int(x) for x in pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text().split(",")]

def readinput():
  return readfile("input.in")

def solution1(data):
  med = median(data)
  return sum([abs(x-med) for x in data])

def calculate_fuel(startpoint, endpoint):
  res = 0
  for i in range(abs(startpoint-endpoint)+1):
    res += i
  return res

def calculate_total_fuel(endpoint, data):
  return sum([calculate_fuel(x, endpoint) for x in data])

def search(left, right, data):
  sum_left = calculate_total_fuel(left, data)
  sum_right = calculate_total_fuel(right, data)

  if (left+1 >= right):
    return min(sum_left, sum_right)
  else:
    sum_left = calculate_total_fuel(left, data)
    sum_right = calculate_total_fuel(right, data)
    mid = int(left + (right - left) / 2)
    return search(mid, right, data) if sum_left > sum_right else search(left, mid, data)

def solution2(data):
  return search(0, len(data)-1, data)

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {solution1(data)}")
  print(f"Solution 2: {solution2(data)}")