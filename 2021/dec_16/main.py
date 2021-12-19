import os
import pathlib
import operator
import functools

def readfile(file):
 return hex_to_bin(pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text())

def readinput():
  return readfile("input.in")

def bin_to_int(bin):
  return int(bin, 2)
  
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
  
  # First three bits is the version
  version_sum = int(packet[0:3], 2)
  print(f"Version: {int(packet[0:3], 2)}")

  # Next three bits is packet type ID
  type_id = int(packet[3:6], 2)
  print(f"Type ID: {type_id}")

  packet_idx = 6
  # Parse type
  if type_id == 4:
    literal_value, end_idx = get_literal_value(packet[6:])
    print(f"Literal value: {literal_value}")
    packet_idx += end_idx
  else:
    packet_idx += 16 if packet[6] == '0' else 12
  
  return version_sum + calculate_version_sum(packet[packet_idx:])

def evaluate_expression(packet):
  if len(packet) < 7:
    return (0,0)
  
  # First three bits is the version (skip)
  # Next three bits is packet type ID
  type_id = int(packet[3:6], 2)

  packet_idx = 6
  # Parse type
  if type_id == 4:
    literal_value, end_idx = get_literal_value(packet[6:])
    return (literal_value, packet_idx+end_idx)
  else:
    packet_idx += 1

    values = []
    if packet[6] == '0':
      sub_packet_length = bin_to_int(packet[packet_idx:packet_idx+15])
      packet_idx += 15
      end_idx = packet_idx + sub_packet_length
      while packet_idx < end_idx:
        val, packet_length = evaluate_expression(packet[packet_idx:packet_idx+sub_packet_length])
        values.append(val)
        packet_idx += packet_length
      pass
    else:
      nr_packets = bin_to_int(packet[packet_idx:packet_idx+11])
      packet_idx += 11
      for _ in range(nr_packets):
        val, packet_length = evaluate_expression(packet[packet_idx:])
        values.append(val)
        packet_idx += packet_length

    # Evaluate expression for each sub packet
    if type_id == 0: # sum
      return (functools.reduce(operator.add, values), packet_idx)
    elif type_id == 1: # product
      return (functools.reduce(operator.mul, values, 1), packet_idx)
    elif type_id == 2: # min
      return (min(values), packet_idx)
    elif type_id == 3: # max
      return (max(values), packet_idx)
    elif type_id == 5: # greater than
      return (1 if values[0] > values[1] else 0, packet_idx)
    elif type_id == 6: # less than
      return (1 if values[0] < values[1] else 0, packet_idx)
    elif type_id == 7: # equal to
      return (1 if values[0] == values[1] else 0, packet_idx)
    else: 
      print(f"Error: Unknown packet type: {type_id}")
      exit(1)

def solution1(data):
  return evaluate_expression(data)

def solution2(data):
  return evaluate_expression(data)[0]

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {solution1(data)}")
  print(f"Solution 2: {solution2(data)}")