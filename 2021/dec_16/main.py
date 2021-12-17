import os
import pathlib

def readfile(file):
 return hex_to_bin(pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text())

def readinput():
  return readfile("input.in")

def hex_to_bin(hex):
  int_value = int(hex, 16)
  width = len(hex) * 4
  return str(bin(int_value))[2:].zfill(width)

def get_literal_value(packet):
  bin_val = ''
  read_index = 0
  while packet[read_index] != '0':
    bin_val += packet[read_index+1:read_index+5]
    read_index += 5

  # Don't forget the last group
  bin_val += packet[read_index+1:read_index+5]

  return (int(bin_val, 2), read_index+5)

def calculate_version_sum(packet):
  if len(packet) < 7:
    return 0

  print(packet)
  version_sum = 0
  
  # First three bits is version
  version_sum += int(packet[0:3], 2)
  print(f"Version: {int(packet[0:3], 2)}")

  # Next three bits is packet type ID
  type_id = int(packet[3:6], 2)
  print(f"Type ID: {type_id}")

  next_idx = 6
  # Parse literal value
  if type_id == 4:
    literal_value, end_idx = get_literal_value(packet[6:])
    next_idx += end_idx
    print(f"Literal value: {literal_value}")
  else:
    # 15 bit number represents number of bits in sub packets
    if packet[6] == '0':
      next_idx += 16
    # 11 bit number represents number of bits in sub packets
    else:
      next_idx += 12
  
  version_sum += calculate_version_sum(packet[next_idx:])

  return version_sum

def solution1(data):
  return calculate_version_sum(data)

def solution2(data):
  return data

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {solution1(data)}")
  #print(f"Solution 2: {solution2(data)}")