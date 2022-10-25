import unittest

from ape_index_lib import ApeIndex


class TestApeIndex(unittest.TestCase):
    def test(self):
        a = ApeIndex
        self.assertEqual(1.0, a.get_ape_index(200, 200))


if __name__ == "__main__":
    unittest.main()
