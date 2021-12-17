import unittest
from main import calculate_version_sum, get_literal_value, hex_to_bin, readfile, solution1, solution2

class TestMethods(unittest.TestCase):

  def setUp(self):
    pass #self.data = readfile("example.in")

  def test_hex_to_bin(self):
    self.assertEqual(hex_to_bin("D2FE28"), "110100101111111000101000")
    self.assertEqual(hex_to_bin("38006F45291200"), "00111000000000000110111101000101001010010001001000000000")
    self.assertEqual(hex_to_bin("EE00D40C823060"), "11101110000000001101010000001100100000100011000001100000")

  def test_get_literal_value(self):
    self.assertEqual(get_literal_value("101111111000101000")[0], 2021)

  def test_calculate_version_sum(self):
    self.assertEqual(calculate_version_sum(hex_to_bin("D2FE28")), 6)
    self.assertEqual(calculate_version_sum(hex_to_bin("EE00D40C823060")), 14  )
    self.assertEqual(calculate_version_sum(hex_to_bin("8A004A801A8002F478")), 16)
    self.assertEqual(calculate_version_sum(hex_to_bin("620080001611562C8802118E34")), 12)
    self.assertEqual(calculate_version_sum(hex_to_bin("C0015000016115A2E0802F182340")), 23)
    self.assertEqual(calculate_version_sum(hex_to_bin("A0016C880162017C3686B18A3D4780")), 31)

  def test_example2(self):
    self.assertTrue
    #self.assertEqual(solution2(self.data), "")

if __name__ == '__main__':
  unittest.main()