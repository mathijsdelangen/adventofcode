import os
import pathlib

def readfile(file):
  return [list(x) for x in pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text().splitlines()]

def readinput():
  return readfile("input.in")

def calculate_points_illegal(character):
  if character == ')':
    return 3
  elif character == ']':
    return 57
  elif character == '}':
    return 1197
  elif character == '>':
    return 25137

def calculate_points_autocomplete(stack):
  points = 0
  while len(stack) > 0:
    character = stack.pop()
    if character == '(':
      points = points * 5 + 1
    elif character == '[':
      points = points * 5 + 2
    elif character == '{':
      points = points * 5 + 3
    elif character == '<':
      points = points * 5 + 4
  return points

def calculate_solutions(data):
  points_sol1 = 0
  points_sol2 = []
  for characters in data:
    stack = []
    is_illegal = False
    for character in characters:
      if character == '[' or character == '(' or character == '{' or character == '<':
        stack.append(character)
      elif stack[-1] == '(':
        if character == ')':
          stack.pop()
        else:
          print(f"{characters} - Expected ), but found {character} instead")
          points_sol1 += calculate_points_illegal(character)
          is_illegal = True
          break
      elif stack[-1] == '[':
        if character == ']':
          stack.pop()
        else:
          print(f"{characters} - Expected ], but found {character} instead")
          points_sol1 += calculate_points_illegal(character)
          is_illegal = True
          break
      elif stack[-1] == '{':
        if character == '}':
          stack.pop()
        else:
          print(f"{characters} - Expected ., but found {character} instead")
          points_sol1 += calculate_points_illegal(character)
          is_illegal = True
          break
      elif stack[-1] == '<':
        if character == '>':
          stack.pop()
        else:
          print(f"{characters} - Expected >, but found {character} instead")
          points_sol1 += calculate_points_illegal(character)
          is_illegal = True
          break
    
    if not is_illegal:
      points_sol2.append(calculate_points_autocomplete(stack))
  
  points_sol2.sort()

  return points_sol1, points_sol2[int(len(points_sol2)/2)]

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {calculate_solutions(data)[0]}")
  print(f"Solution 2: {calculate_solutions(data)[1]}")