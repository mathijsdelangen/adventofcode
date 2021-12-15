import os
import pathlib
import sys

def readfile(file):
  return [[int(x) for x in list(line)] for line in pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text().splitlines()]

def readinput():
  return readfile("input.in")

def calculate_min_risk_map(risk_levels):
  # Assume square
  max_index = len(risk_levels)-1
  min_risk_map = [[-1 for x in range(max_index+1)] for y in range(max_index+1)]
  min_risk_map[max_index][max_index] = 0

  for _ in range(2):
    for idx in reversed(range(max_index+1)):
      for y in reversed(range(idx+1)):
        for x in reversed(range(idx+1)):
          risk = 0 if x == max_index and y == max_index else sys.maxsize

          # For the second iteration
          if y > 0 and min_risk_map[y-1][x] != -1:
            risk = min(risk, min_risk_map[y-1][x] + risk_levels[y-1][x])
          if x > 0 and min_risk_map[y][x-1] != -1:
            risk = min(risk, min_risk_map[y][x-1] + risk_levels[y][x-1])

          if y < max_index:
            risk = min(risk, min_risk_map[y+1][x] + risk_levels[y+1][x])
          if x < max_index:
            risk = min(risk, min_risk_map[y][x+1] + risk_levels[y][x+1])
          
          min_risk_map[y][x] = risk

  return min_risk_map

def wrap(val):
  while val > 9:
    val = val-9 # Wrap around 10 -> 1
  return val

def convert_map(risk_levels):
  len_original = len(risk_levels)
  updated_risk_levels = [[0 for x in range(len_original*5)] for y in range(len_original*5)]
  for y in range(5*len_original):
    for x in range(5*len_original):
      y_mult = int(y / len_original)
      y_orig = y % len_original
      x_mult = int(x / len_original)
      x_orig = x % len_original
      updated_risk_levels[y][x] = wrap(risk_levels[y_orig][x_orig] + (x_mult + y_mult))

  return updated_risk_levels

def solution1(data):
  min_risk_map = calculate_min_risk_map(data)
  return min_risk_map[0][0]

def solution2(data):
  min_risk_map = calculate_min_risk_map(convert_map(data))
  return min_risk_map[0][0]

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {solution1(data)}")
  print(f"Solution 2: {solution2(data)}")