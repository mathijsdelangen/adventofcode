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

def solution2(data):
  return data

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {solution1(data)}")
  #print(f"Solution 2: {solution2(data)}")