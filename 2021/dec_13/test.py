import unittest
from main import readfile, solution1, solution2

class TestMethods(unittest.TestCase):

  def setUp(self):
    self.data = readfile("example.in")

  def test_example1(self):
    self.assertEqual(solution1(*self.data), 17)

  def test_example2(self):
    solution2(*self.data)
    # Inspect output manually

if __name__ == '__main__':
  unittest.main()