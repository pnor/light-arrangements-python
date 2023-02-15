#!/usr/bin/env python3

import light_arrangements_python
import unittest
import numpy as np

NUMBER_LIGHTS = 100


def get_light_arrangements():
    return light_arrangements_python.init_test(
        2, "./tests/positions2d.csv", 0.03, (0.5, 0.5, 2), [0, 1, 2]
    )


class TestFill(unittest.TestCase):
    def test_fill(self):
        color = (100, 100, 100)
        arr = get_light_arrangements()
        arr.fill(color)
        for i in range(NUMBER_LIGHTS):
            self.assertTrue(arr.get_by_index(i) == color)

        color = (20, 123, 40)
        arr.fill(color)
        for i in range(NUMBER_LIGHTS):
            assert arr.get_by_index(i) == color


class GetSet(unittest.TestCase):
    def test_get_and_set_by_index(self):
        color = (100, 100, 100)
        arr = get_light_arrangements()
        arr.set_by_index(2, color)
        assert arr.get_by_index(2) == color

    def test_bad_index(self):
        arr = get_light_arrangements()
        with self.assertRaises(ValueError):
            arr.set_by_index(1000, (1, 2, 3))
        with self.assertRaises(ValueError):
            arr.get_by_index(1000)

        with self.assertRaises(OverflowError):
            arr.set_by_index(-1, (1, 2, 3))
        with self.assertRaises(OverflowError):
            arr.get_by_index(-50)


class GetSetNear(unittest.TestCase):
    def test_get_set_near(self):
        arr = get_light_arrangements()
        color = (100, 100, 100)
        arr.set_closest([0.5, 0.5], 0.2, color)
        self.assertTrue(arr.get_closest([0.5, 0.5], 0.2), color)

        color = (100, 100, 100)
        arr.set_closest([0.2, 0.9], 0.2, color)
        self.assertTrue(arr.get_closest([0.2, 0.9], 0.2), color)

    def test_get_set_none(self):
        arr = get_light_arrangements()
        arr.set_closest([10, 10], 0.2, (0, 0, 0))
        self.assertTrue(arr.get_closest([10, 10], 0.2) == None)


class SetDecreasingIntensity(unittest.TestCase):
    def test_decreasing_intensity(self):
        arr = get_light_arrangements()
        color = [255, 255, 255]
        arr.set_decreasing_intensity([0.5, 0.5], 0.2, color)
        self.assertTrue(arr.get_closest([0.5, 0.5], 0.2)[0] != 0)

    def test_decreasing_intensity_merge(self):
        arr = get_light_arrangements()
        color = [255, 255, 255]
        arr.set_decreasing_intensity_merge([0.5, 0.5], 0.2, color)
        self.assertTrue(arr.get_closest([0.5, 0.5], 0.2)[0] != 0)


class SetBox(unittest.TestCase):
    def test_box(self):
        arr = get_light_arrangements()
        color = [255, 255, 255]
        arr.set_all_in_box([0.0, 0.0], [1, 1], color)
        self.assertTrue(arr.get_closest([0.5, 0.5], 0.2)[0] == 255)


class SetRadius(unittest.TestCase):
    def test_radius(self):
        arr = get_light_arrangements()
        color = [255, 255, 255]
        arr.set_all_in_radius([0.5, 0.5], 0.5, color)
        self.assertTrue(arr.get_closest([0.5, 0.5], 0.2)[0] == 255)


class ColorTypes(unittest.TestCase):
    def test_accepts_lists_for_colors(self):
        arr = get_light_arrangements()
        color = [100, 100, 100]
        arr.set_closest([0.5, 0.5], 0.2, color)
        arr.set_by_index(0, color)
        arr.set_decreasing_intensity([0, 0], 0.2, color)
        arr.set_decreasing_intensity_merge([0, 0], 0.2, color)
        arr.set_all_in_radius((0.5, 0.5), 0.2, color)
        arr.set_all_in_box((0.5, 0.5), (0.7, 0.7), color)

    def test_accepts_numpy_array_for_colors(self):
        arr = get_light_arrangements()
        color = np.array([10, 20, 30])
        arr.set_closest([0.5, 0.5], 0.2, color)
        arr.set_by_index(0, color)
        arr.set_decreasing_intensity([0, 0], 0.2, color)
        arr.set_decreasing_intensity_merge([0, 0], 0.2, color)
        arr.set_all_in_radius((0.5, 0.5), 0.2, color)
        arr.set_all_in_box((0.5, 0.5), (0.7, 0.7), color)


class BadColor(unittest.TestCase):
    def test_too_big_colors(self):
        arr = get_light_arrangements()
        with self.assertRaises(OverflowError):
            arr.set_by_index(2, (256, 256, 256))
        with self.assertRaises(OverflowError):
            arr.set_by_index(2, (300, 0, 0))

    def test_negative_colors(self):
        arr = get_light_arrangements()
        with self.assertRaises(OverflowError):
            arr.set_by_index(0, (-1, -1, -1))
        with self.assertRaises(OverflowError):
            arr.set_by_index(0, (0, -1, 0))

    def test_float_colors(self):
        arr = get_light_arrangements()
        with self.assertRaises(TypeError):
            arr.set_by_index(0, (2.5, 3.5, 0.334))


class BadDimensions(unittest.TestCase):
    def test_mixed_dimensions(self):
        arr = get_light_arrangements()
        color = (100, 100, 100)
        with self.assertRaises(ValueError):
            arr.set_closest([0.1], 0.2, color)
        with self.assertRaises(ValueError):
            arr.set_closest([0.1, 0.2, 0.3], 0.2, color)
        with self.assertRaises(ValueError):
            arr.set_closest([], 0.0, color)


class Show(unittest.TestCase):
    def test_show(self):
        arr = get_light_arrangements()
        arr.show()


class NumberLights(unittest.TestCase):
    def test_number(self):
        arr = get_light_arrangements()
        self.assertTrue(arr.number_lights(), 100)


if __name__ == "__main__":
    unittest.main()
