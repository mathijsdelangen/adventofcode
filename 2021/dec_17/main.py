import os
import pathlib

def readfile(file):
  text = pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text()
  text = text.replace("target area: ","")
  text = text.split(",")
  bounds_x = text[0].replace("x=","").split("..")
  bounds_y = text[1].replace("y=","").split("..")
  return (range(int(bounds_x[0]),int(bounds_x[1])), range(int(bounds_y[0]), int(bounds_y[1])))

def in_target_range(x, y, range_x, range_y):
  return x in range_x and y in range_y

def exceeded_target_range(x, y, range_x, range_y):
  return x >= range_x.stop or y > range_y.stop

def velocity_x_ends_up_in_target(x, range_x):
  pos_x = 0
  vel_x = x
  while vel_x > 0 and pos_x < range_x.stop:
    pos_x += vel_x
    if pos_x in range_x:
      return True
    vel_x -= 1

  return False

def velocity_y_ends_up_in_target(y, range_y):
  pos_y = 0
  vel_y = y
  while pos_y > range_y.stop:
    pos_y += vel_y
    if pos_y in range_y:
      return True
    vel_y -= 1

  return False

def velocity_xy_ends_up_in_target(x, y, range_x, range_y):
  pos_x = 0
  pos_y = 0
  vel_x = x
  vel_y = y
  while pos_y > range_y.stop and pos_x < range_x.stop:
    pos_x += vel_x
    pos_y += vel_y
    if pos_x in range_x and pos_y in range_y:
      return True
    vel_x = max(0, vel_x-1)
    vel_y -= 1

  return False

def get_highest_point(y):
  pos_y = 0
  vel_y = y
  max_y = -1
  while pos_y >= max_y:
    pos_y += vel_y
    vel_y -= 1
    max_y = max(pos_y, max_y)

  return max_y

def readinput():
  return readfile("input.in")

def solution1(data):
  target_range_x = data[0]
  target_range_y = data[1]

  valid_x_ranges = []
  for x in range(target_range_x.stop):
    if velocity_x_ends_up_in_target(x, target_range_x):
      valid_x_ranges.append(x)

  valid_y_ranges = []
  for y in range(1000):
    if velocity_y_ends_up_in_target(y, target_range_y):
      valid_y_ranges.append(y)

  # Find velocities that end up in the target range
  max_max_heigth = 0
  for x in valid_x_ranges:
    for y in valid_y_ranges:
      if velocity_xy_ends_up_in_target(x, y, target_range_x, target_range_y):
        max_max_heigth = max(get_highest_point(y), max_max_heigth)

  return max_max_heigth

def solution2(data):
  return data

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {solution1(data)}")
  #print(f"Solution 2: {solution2(data)}")