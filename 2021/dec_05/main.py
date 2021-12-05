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

def create_coordinates(line_segments):
  max_x = 0
  max_y = 0
  for segment in line_segments:
    if segment.get_x1() > max_x:
      max_x = segment.get_x1()
    if segment.get_y1() > max_y:
      max_y = segment.get_y1()

  return [[0 for x in range(max_x+2)] for y in range(max_y+2)] 

def update_coords_with_horizontal_lines(coordinates, line_segments):
  for segment in line_segments:
    if segment.get_x1() == segment.get_x2():
      values = range(segment.get_y1(), segment.get_y2()+1) if segment.get_y1() < segment.get_y2() else range(segment.get_y2(), segment.get_y1()+1)
      for y in values:
        coordinates[y][segment.get_x1()] += 1
      
    if segment.get_y1() == segment.get_y2():
      values = range(segment.get_x1(), segment.get_x2()+1) if segment.get_x1() < segment.get_x2() else range(segment.get_x2(), segment.get_x1()+1)
      for x in values:
        coordinates[segment.get_y1()][x] += 1
  
  return coordinates

def update_coords_with_diagonal_lines(coordinates, line_segments):
  for segment in line_segments:
    if abs(segment.get_x1() - segment.get_x2()) == abs(segment.get_y1() - segment.get_y2()):
      segment_length = abs(segment.get_x1() - segment.get_x2()) + 1
      slope_x = int(((segment.get_x2() - segment.get_x1())) / (segment_length - 1))
      slope_y = int(((segment.get_y2() - segment.get_y1())) / (segment_length - 1))
      start_x = segment.get_x1()
      start_y = segment.get_y1()
      for step in range(0, segment_length):
        coordinates[start_y + step * slope_y][start_x + step * slope_x] += 1

  return coordinates

def calculate_sum(coordinates):
  sum = 0
  for y in range(0,len(coordinates)):
    for x in range(0, len(coordinates[0])):
      if coordinates[y][x] >= 2:
        sum += 1

  return sum

def solution1(data):
  coordinates = create_coordinates(data)
  coordinates = update_coords_with_horizontal_lines(coordinates, data)
  return calculate_sum(coordinates)

def solution2(data):
  coordinates = create_coordinates(data)
  coordinates = update_coords_with_horizontal_lines(coordinates, data)
  coordinates = update_coords_with_diagonal_lines(coordinates, data)
  return calculate_sum(coordinates)

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {solution1(data)}")
  print(f"Solution 2: {solution2(data)}")