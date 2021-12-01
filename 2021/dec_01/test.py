import unittest
from dec_01 import ReadFile, CalculateSolution

class TestMethods(unittest.TestCase):

  def test_example1(self):
    sol = CalculateSolution(ReadFile("inputs/dec_01_example.in"))
    #self.assertEqual(sol, 7)

  def test_example2(self):
    sol = CalculateSolution(ReadFile("inputs/dec_01_example.in"))
    self.assertEqual(sol, 5)

if __name__ == '__main__':
  unittest.main()