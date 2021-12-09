import os
import pathlib

def readfile(file):
  return [[int(x) for x in list(line)] for line in pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text().splitlines()]
  
def readinput():
  return readfile("input.in")

def is_low_point(x,y,data):
  height = data[y][x]
  result = True
  if x > 0:
    result &= height < data[y][x-1]
  if x < len(data[0])-1:
    result &= height < data[y][x+1]
  if y > 0:
    result &= height < data[y-1][x]
  if y < len(data)-1:
    result &= height < data[y+1][x]

  return result

def solution1(data):
  result = 0
  for y in range(len(data)):
    for x in range(len(data[y])):
      if is_low_point(x, y, data):
        result += data[y][x]+1
  return result

def solution2(data):
  return data

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {solution1(data)}")
  #print(f"Solution 2: {solution2(data)}")