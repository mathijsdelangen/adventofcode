import unittest
from main import calculate_solutions, readfile

class TestMethods(unittest.TestCase):

  def setUp(self):
    self.data = readfile("example.in")

  def test_example1(self):
    self.assertEqual(calculate_solutions(self.data)[0], 26397)

  def test_example2(self):
    self.assertEqual(calculate_solutions(self.data)[1], 288957)

if __name__ == '__main__':
  unittest.main()