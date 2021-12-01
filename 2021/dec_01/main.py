import os
import pathlib

def readfile(file):
  return [int(x) for x in pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text().splitlines()]

def readinput():
  return readfile("input.in")

def solution(depths):
  count = 0
  for i in range(3, len(depths)):
    if depths[i]+depths[i-1]+depths[i-2] > depths[i-1]+depths[i-2]+depths[i-3]:
      count += 1

  return count

if __name__ == '__main__':
  sol = solution(readinput())
  print(f"Total number of depth increases: {sol}")