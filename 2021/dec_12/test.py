import unittest
from main import readfile, solution1, solution2

class TestMethods(unittest.TestCase):

  def setUp(self):
    self.data = readfile("example.in")
    self.data2 = readfile("example2.in")
    self.data3 = readfile("example3.in")

  def test_example1(self):
    self.assertEqual(solution1(self.data), 10)

  def test_example1_2(self):
    self.assertEqual(solution1(self.data2), 19)

  def test_example1_3(self):
    self.assertEqual(solution1(self.data3), 226)

  def test_example2(self):
    self.assertTrue
    #self.assertEqual(solution2(self.data), "")

if __name__ == '__main__':
  unittest.main()