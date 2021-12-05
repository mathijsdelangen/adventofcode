import os
import pathlib

class Board():
  def __init__(self, values, size = 5):
    self.board_size = size
    self.values = values
  
  def get_values(self):
    return self.values

  def remove_value(self, value):
    for i in range(0, self.board_size):
      for j in range(0, self.board_size):
        if self.values[i][j] == value:
          self.values[i][j] = -1
    pass

  def has_bingo(self):
    # Check horizontal
    for i in range(0, self.board_size):
      if all(x == -1 for x in self.values[i]):
        return True

    # Check vertical
    transposed = list(map(list, zip(*self.values)))
    for i in range(0, self.board_size):
      if all(x == -1 for x in transposed[i]):
        return True

    return False

  def sum_all(self):
    sum = 0
    for i in range(0, self.board_size):
      for j in range(0, self.board_size):
        if self.values[i][j] != -1:
          sum += self.values[i][j]

    return sum

def readfile(file):
  infile = pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text().splitlines()
  list_sequence = [int(x) for x in infile[0].split(",")]
  
  boards = []
  for i in range(2, len(infile), 6):
    values = []
    for j in range(i,i+5):
      values.append([int(x) for x in infile[j].split()])
    boards.append(Board(values))

  return (list_sequence, boards)

def readinput():
  return readfile("input.in")

def solution1(data):
  list_sequence, boards = data
  
  for bingo_nr in list_sequence:
    for board in boards:
      board.remove_value(bingo_nr)
      if board.has_bingo():
        return board.sum_all() * bingo_nr

def solution2(data):
  return data

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {solution1(data)}")
  #print(f"Solution 2: {solution2(data)}")