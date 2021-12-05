import unittest
from main import Board, readfile, solution1, solution2

class TestMethods(unittest.TestCase):

  def setUp(self):
    self.data = readfile("example.in")

  def test_bingo_horizontal(self):
    test_values = [[1,2,3],[-1,-1,-1],[7,8,9]]
    b = Board(test_values, 3)
    self.assertTrue(b.has_bingo())

  def test_bingo_vertical(self):
    test_values = [[-1,2,3],[-1,5,6],[-1,8,9]]
    b = Board(test_values, 3)
    self.assertTrue(b.has_bingo())

  def test_no_bingo(self):
    test_values = [[1,2,3],[4,5,6],[7,8,9]]
    b = Board(test_values, 3)
    self.assertFalse(b.has_bingo())

  def test_example1(self):
    self.assertEqual(solution1(self.data), 4512)

  def test_example2(self):
    self.assertEqual(solution2(self.data), 1924)

if __name__ == '__main__':
  unittest.main()