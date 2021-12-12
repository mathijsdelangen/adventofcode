import os
import pathlib

def readfile(file):
  return [x.split("-") for x in pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text().splitlines()]

def readinput():
  return readfile("input.in")

def find_all_paths(current_point, transitions):
  solutions = [[current_point]]

  updated = True
  count = 0
  while updated:
    count += 1 
    updated = False

    new_solutions = []
    for s in solutions:
      valid_transitions = []
      for transition in transitions:
        if s[-1] == "end":
          pass #print(f"Skip, because we already reached the end point for {s}")  
        elif s[-1] in transition:
          next_point = transition[1] if s[-1] == transition[0] else transition[0]

          if str.islower(next_point) and next_point in s:
            # Skip, not allowed
            pass #print(f"Skip transition to {next_point}, because it is already in {s}")  
          else:
            pass #print(f"Append {next_point} to valid transitions (current solution {s})")
            valid_transitions.append(next_point)
        
      if len(valid_transitions) > 0:
        for t in valid_transitions:
          new_solutions.append(s+[t])
        updated = True
      else:
        new_solutions.append(s)

    solutions = new_solutions  
  
  return solutions

def solution1(data):
  paths = find_all_paths("start", data)
  solution = [x for x in paths if x[-1] == "end"]

  return len(solution)

def solution2(data):
  return data

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {solution1(data)}")
  #print(f"Solution 2: {solution2(data)}")