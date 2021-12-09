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

def mark_appending_locations(x, y, data, map, lowest_point):
  if x > 0:
    if data[y][x-1] != 9 and map[y][x-1] == (-1,-1):
      map[y][x-1] = lowest_point
      map = mark_appending_locations(x-1, y, data, map, lowest_point)
  if x < len(data[0])-1:
    if data[y][x+1] != 9 and map[y][x+1] == (-1,-1):
      map[y][x+1] = lowest_point
      map = mark_appending_locations(x+1, y, data, map, lowest_point)
  if y > 0:
    if data[y-1][x] != 9 and map[y-1][x] == (-1,-1):
      map[y-1][x] = lowest_point
      map = mark_appending_locations(x, y-1, data, map, lowest_point)
  if y < len(data)-1:
    if data[y+1][x] != 9 and map[y+1][x] == (-1,-1):
      map[y+1][x] = lowest_point
      map = mark_appending_locations(x, y+1, data, map, lowest_point)
  
  return map

def solution1(data):
  result = 0
  for y in range(len(data)):
    for x in range(len(data[y])):
      if is_low_point(x, y, data):
        result += data[y][x]+1
  return result

def solution2(data):
  low_points = []
  map = [[(-1,-1) for x in range(len(data[0]))] for y in range(len(data))]
  for y in range(len(data)):
    for x in range(len(data[y])):
      if is_low_point(x, y, data):
        map[y][x] = (x,y)
        low_points.append((x,y))
  
  for low_point in low_points:
    mark_appending_locations(low_point[0], low_point[1], data, map, low_point)
  
  flattened_map = [item for sublist in map for item in sublist]
  sizes = [ flattened_map.count(low_point) for low_point in low_points ]
  sizes.sort(reverse=True)
  
  return sizes[0]*sizes[1]*sizes[2]

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {solution1(data)}")
  print(f"Solution 2: {solution2(data)}")