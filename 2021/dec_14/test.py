import unittest
from main import readfile, solution

class TestMethods(unittest.TestCase):

  def setUp(self):
    self.data = readfile("example.in")

  def test_example1(self):
    self.assertEqual(solution(*self.data, 10), 1588)

  def test_example1_1(self):
    self.assertEqual(solution(*self.data, 1), 1)

  def test_example1_2(self):
    self.assertEqual(solution(*self.data, 2), 5)

  def test_example1_3(self):
    self.assertEqual(solution(*self.data, 3), 7)

  def test_example2(self):
    self.assertEqual(solution(*self.data, 40), 2188189693529)

if __name__ == '__main__':
  unittest.main()