import unittest
from main import readfile, solution

class TestMethods(unittest.TestCase):

  def setUp(self):
    self.data = readfile("example.in")

  def test_example1(self):
    sol = solution(self.data)
    #self.assertEqual(sol, 7)

  def test_example2(self):
    sol = solution(self.data)
    self.assertEqual(sol, 5)

if __name__ == '__main__':
  unittest.main()