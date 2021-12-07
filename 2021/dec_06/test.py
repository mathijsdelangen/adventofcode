import unittest
from main import readfile, calculate_nr_fish

class TestMethods(unittest.TestCase):

  def setUp(self):
    self.data = readfile("example.in")

  def test_example1_18_days(self):
    self.assertEqual(calculate_nr_fish(self.data, 18), 26)

  def test_example1_80_days(self):
    self.assertEqual(calculate_nr_fish(self.data, 80), 5934)

  def test_example2(self):
    self.assertEqual(calculate_nr_fish(self.data, 256), 26984457539)

if __name__ == '__main__':
  unittest.main()