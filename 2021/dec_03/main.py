import os
import operator
import pathlib

def readfile(file):
  return [x for x in pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text().splitlines()]

def readinput():
  return readfile("input.in")

def solution1(data):
  most_common_values = ""

  for bit in range(len(data[0])):
    count = 0
    for line in data:
      if line[bit] == "0":
        count += 1
    
    most_common_values += "0" if count > len(data) / 2 else "1"

  gamma = int(most_common_values,2) # convert binary to int
  epsilon = ~gamma & pow(2,len(data[0]))-1 # inverse gamma

  return gamma * epsilon

def solution2(data):
  rating_oxygen_generator = calculate(data, most_common)
  rating_co2_scrubber = calculate(data, least_common)
  return rating_oxygen_generator * rating_co2_scrubber

def most_common(left, right):
  return right if len(right) >= len(left) else left

def least_common(left, right):
  return left if len(left) <= len(right) else right

def calculate(data, op, idx=0):
  if len(data) == 1:
    return int(data[0],2)

  ones = []
  zeros = []
  for d in data:
    zeros.append(d) if d[idx] == "0" else ones.append(d)     
  
  return calculate(op(zeros, ones), op, idx+1)
  
if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {solution1(data)}")
  print(f"Solution 2: {solution2(data)}")