import unittest
from main import decode, readfile, solution1, solution2

class TestMethods(unittest.TestCase):

  def setUp(self):
    self.data = readfile("example.in")

  def test_example1(self):
    self.assertEqual(solution1(self.data), 26)

  def test_decode(self):
    self.assertEqual(decode(self.data[0][0],self.data[0][1]), 8394)
    self.assertEqual(decode(self.data[1][0],self.data[1][1]), 9781)
    self.assertEqual(decode(self.data[2][0],self.data[2][1]), 1197)
    self.assertEqual(decode(self.data[3][0],self.data[3][1]), 9361)
    self.assertEqual(decode(self.data[4][0],self.data[4][1]), 4873)
    self.assertEqual(decode(self.data[5][0],self.data[5][1]), 8418)
    self.assertEqual(decode(self.data[6][0],self.data[6][1]), 4548)
    self.assertEqual(decode(self.data[7][0],self.data[7][1]), 1625)
    self.assertEqual(decode(self.data[8][0],self.data[8][1]), 8717)
    self.assertEqual(decode(self.data[9][0],self.data[9][1]), 4315)

  def test_example2(self):
    self.assertEqual(solution2(self.data), 61229)

if __name__ == '__main__':
  unittest.main()