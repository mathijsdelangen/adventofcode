import os
import pathlib

def readfile(file):
  coordinates = []
  folds = []
  lines = pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text().splitlines()
  for line in lines:
    if not line:
      pass
    elif line.startswith("fold"):
      folds.append(line.replace("fold along ", ""))
    else:
      coordinates.append([int(c) for c in line.split(",")])

  # Determine map bounds
  max_x = max([c[0] for c in coordinates])
  max_y = max([c[1] for c in coordinates])

  # Create map
  map = [[False for x in range(max_x+1)] for y in range(max_y+1)]

  # Fill map
  for (x,y) in coordinates:
    map[y][x] = True

  return (map, folds)

def readinput():
  return readfile("input.in")

def count_dots(map):
  return sum([row.count(True) for row in map])

def fold_map(map, fold):
  # Determine fold values
  orientation, location = fold.split("=")
  loc = int(location)

  len_x = len(map[0])
  len_y = len(map)

  if orientation == 'y': # Horizontal fold
    # Create new map after fold
    max_y = max(len_y-loc-1, len_y-(len_y-loc))
    new_map = [[False for x in range(len_x)] for y in range(max_y)]
    # Fill new map
    for y in range(max_y):
      for x in range(len_x):
        value = map[loc-(loc-y)][x] or map[loc+(loc-y)][x]
        new_map[y][x] = value

  else: # Vertical fold
    # Create new map after fold
    max_x = max(len_x-loc-1, len_x-(len_x- loc))
    new_map = [[False for x in range(max_x)] for y in range(len_y)]
    # Fill new map
    for y in range(len_y):
      for x in range(max_x):
        value = map[y][loc-(loc-x)] or map[y][loc+(loc-x)]
        new_map[y][x] = value

  return new_map

def print_map(map):
  print("") # Clear console
  
  for y in range(len(map)):
    out_str = ""
    for x in range(len(map[0])):
      out_str += "#" if map[y][x] else "."
    print(out_str)

def solution1(map, folds):
  map = fold_map(map, folds[0])
  return count_dots(map)

def solution2(map, folds):
  for fold in folds:
    map = fold_map(map, fold)

  print_map(map)

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {solution1(*data)}")
  print(f"Solution 2: {solution2(*data)}")