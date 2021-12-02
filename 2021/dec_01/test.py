import unittest
from main import readfile, solution1, solution2

class TestMethods(unittest.TestCase):

  def setUp(self):
    self.data = readfile("example.in")

  def test_example1(self):
    sol = solution1(self.data)
    self.assertEqual(sol, 7)

  def test_example2(self):
    sol = solution2(self.data)
    self.assertEqual(sol, 5)

if __name__ == '__main__':
  unittest.main()