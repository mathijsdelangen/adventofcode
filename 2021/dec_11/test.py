import unittest
from main import readfile, solution

class TestMethods(unittest.TestCase):

  def setUp(self):
    self.data = readfile("example.in")

  def test_example1(self):
    pass
    #self.assertEqual(solution1(self.data), 1656)

  def test_example2(self):
    self.assertEqual(solution(self.data), 195)

if __name__ == '__main__':
  unittest.main()