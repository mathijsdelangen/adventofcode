import unittest
from main import convert_map, readfile, solution1, solution2

class TestMethods(unittest.TestCase):

  def setUp(self):
    self.data = readfile("example.in")
    self.data2 = readfile("example2.in")

  def test_example1(self):
    self.assertEqual(solution1(self.data), 40)

  def test_example2_covert_map(self):
    self.assertEqual(convert_map(self.data), self.data2)

  def test_example2_result(self):
    self.assertEqual(solution1(self.data2), 315)

  def test_example2(self):
    self.assertEqual(solution2(self.data), 315)

if __name__ == '__main__':
  unittest.main()