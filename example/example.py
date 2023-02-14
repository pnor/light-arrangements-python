#!/usr/bin/env python3

import light_arrangements_python as light
import time
import numpy as np

obj = light.init_test(2, "./example/positions2d.csv", 0.03, (0.5, 0.5, 2), [0, 1, 2])

obj.fill((25, 25, 25))


obj.set_closest(light.Loc2.cartesian([0.2, 0.2]), 0.2, (255, 0, 0))


obj.show()


time.sleep(1)
p = 0
for i in range(200):
    obj.fill((0, 0, 0))
    p += 0.05
    p %= 2 * np.pi

    loc_a = light.Loc2.polar(0.4, [p], (0.5, 0.5))
    loc_b = light.Loc2.polar(0.4, [(2 * np.pi) - p], (0.5, 0.5))

    obj.set_decreasing_intensity_merge(loc_a, 0.2, (255, 0, 0))
    obj.set_decreasing_intensity_merge(loc_b, 0.2, (0, 255, 0))

    obj.show()
    # time.sleep(0.05)
