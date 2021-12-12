import os
import pathlib

def readfile(file):
  return [[int(x) for x in list(line)] for line in pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text().splitlines()]

def readinput():
  return readfile("input.in")

def increase_level(x, y, data):
  data[y][x] += 1
  if data[y][x] == 10: # First increase > 9, update adjecent tiles
    for adj_y in range(max(y-1,0), min(y+2, len(data))):
      for adj_x in range(max(x-1,0), min(x+2, len(data[0]))):
        data = increase_level(adj_x, adj_y, data)
  return data

def solution(data):
  flashes = 0
  all_flash = False
  count = 0
  while not all_flash:
    # Increase levels
    for y in range(0, len(data)):
      for x in range(0, len(data[0])):
        data = increase_level(x, y, data)
  
    # Flash
    all_flash = True
    for y in range(0, len(data)):
      for x in range(0, len(data[0])):
        if data[y][x] > 9:
          flashes += 1
          data[y][x] = 0
        else:
          all_flash = False
    count += 1      
    
  return count

if __name__ == '__main__':
  data = readinput()
  #print(f"Solution 1: {solution1(data)}")
  print(f"Solution 2: {solution(data)}")