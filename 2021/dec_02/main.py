import os
import pathlib

def readfile(file):
  data = [x.split() for x in pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text().splitlines()]
  return [(x[0], int(x[1])) for x in data]

def readinput():
  return readfile("input.in")

def solution(data):
  aim = 0
  pos = 0
  depth = 0
  
  for instruction, val in data:
    if instruction == "forward":
      pos += val
      depth += val * aim
    if instruction == "down":
      aim += val
    if instruction == "up":
      aim -= val

  return pos * depth

if __name__ == '__main__':
  sol = solution(readinput())
  print(f"Solution: {sol}")