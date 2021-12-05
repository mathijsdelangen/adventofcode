import os
import pathlib

class LineSegment():
  def __init__(self, x1, y1, x2, y2):
    self.x1 = x1
    self.y1 = y1

    self.x2 = x2
    self.y2 = y2
  
  def get_x1(self):
    return self.x1

  def get_y1(self):
    return self.y1

  def get_x2(self):
    return self.x2

  def get_y2(self):
    return self.y2

def readfile(file):
  segments = []
  lines = pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text().splitlines()
  for line in lines:
    endpoints = [int(x) for x in line.replace(" -> ",",").split(",")]
    segments.append(LineSegment(*endpoints))
  return segments

def readinput():
  return readfile("input.in")

def solution1(data):
  # Determine (max) bounds
  max_x = 0
  max_y = 0
  for segment in data:
    if segment.get_x1() > max_x:
      max_x = segment.get_x1()
    if segment.get_y1() > max_y:
      max_y = segment.get_y1()

  # Create empty vent coordinates
  coordinates = [[0 for x in range(max_x+1)] for y in range(max_y+1)] 

  for segment in data:
    if segment.get_x1() == segment.get_x2():
      values = range(segment.get_y1(), segment.get_y2()+1) if segment.get_y1() < segment.get_y2() else range(segment.get_y2(), segment.get_y1()+1)
      for y in values:
        coordinates[y][segment.get_x1()] += 1
      
    if segment.get_y1() == segment.get_y2():
      values = range(segment.get_x1(), segment.get_x2()+1) if segment.get_x1() < segment.get_x2() else range(segment.get_x2(), segment.get_x1()+1)
      for x in values:
        coordinates[segment.get_y1()][x] += 1

  sum = 0
  for y in range(0,len(coordinates)):
    for x in range(0, len(coordinates[0])):
      if coordinates[y][x] >= 2:
        sum += 1

  return sum

def solution2(data):
  return data

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {solution1(data)}")
  #print(f"Solution 2: {solution2(data)}")