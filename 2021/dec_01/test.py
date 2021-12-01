import unittest
from main import readinput, solution

class TestMethods(unittest.TestCase):

  def test_example1(self):
    sol = solution(readinput("inputs/example.in"))
    #self.assertEqual(sol, 7)

  def test_example2(self):
    sol = solution(readinput("inputs/example.in"))
    self.assertEqual(sol, 5)

if __name__ == '__main__':
  unittest.main()