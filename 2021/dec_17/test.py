import unittest
from main import readfile, solution1, solution2, velocity_xy_ends_up_in_target

class TestMethods(unittest.TestCase):

  def setUp(self):
    self.data = readfile("example.in")

  def test_example1(self):
    pass #self.assertEqual(solution1(self.data), 45)

  def test_velocity_in_target_range(self):
    self.assertTrue(velocity_xy_ends_up_in_target(6, 0, self.data[0], self.data[1]))
    self.assertTrue(velocity_xy_ends_up_in_target(6, 3, self.data[0], self.data[1]))

  def test_example2(self):
    self.assertEqual(solution2(self.data), 112)

if __name__ == '__main__':
  unittest.main()