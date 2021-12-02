import unittest
from main import readfile, solution

class TestMethods(unittest.TestCase):

  def setUp(self):
    self.data = readfile("example.in")

  def test_example1(self):
    self.assertEqual(solution(self.data),150)

  def test_example2(self):
    self.assertTrue
    #self.assertEqual(solution(self.data),"")

if __name__ == '__main__':
  unittest.main()