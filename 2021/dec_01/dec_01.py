import os
import pathlib

def ReadFile(file):
  f = open(os.path.join(pathlib.Path(__file__).parent.resolve(),file), 'r')
  lines = f.read().splitlines() 
  lines_as_int = [int(i) for i in lines]
  f.close()
  return lines_as_int

def CalculateSolution(depths):
  count = 0
  for i in range(1, len(depths)):
    if depths[i] > depths[i-1]:
      count += 1

  return count

if __name__ == '__main__':
  sol = CalculateSolution(ReadFile("inputs/dec_01.in"))
  print(f"Total number of depth increases: {sol}")