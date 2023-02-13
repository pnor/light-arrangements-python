#!/usr/bin/env python3

import light_arrangements_python
import time
import numpy as np

obj = light_arrangements_python.init_test(
    2, "./example/positions2d.csv", 0.03, (0.5, 0.5, 2), [0, 1, 2]
)

obj.fill((25, 25, 25))

obj.set_closest_polar(0.1, [0], (0.5, 0.5), 0.2, (255, 0, 0))
obj.set_closest_polar(0.2, [np.pi], (0.5, 0.5), 0.2, (0, 255, 255))
obj.set_closest_polar(0.3, [np.pi / 2], (0.5, 0.5), 0.2, (255, 0, 255))
obj.set_closest_polar(0.4, [np.pi * 1.5], (0.5, 0.5), 0.2, (0, 0, 255))

obj.set_decreasing_intensity_polar(0.4, [np.pi * 1.8], (0.5, 0.5), 0.3, (100, 255, 200))


obj.get_closest_polar(0.2, [40], (0.5, 0.5), 0.2)

obj.set_all_in_box([0, 0], [0.4, 0.4], (150, 150, 0))
obj.set_all_in_radius_polar(0.0, [0], (1, 1), 0.3, (0, 150, 150))
obj.set_by_index(0, (255, 255, 255))
obj.set_by_index(1, (255, 255, 255))
obj.set_by_index(2, (255, 255, 255))
obj.show()

# for i in range(200):
#     obj.show()
#     time.sleep(0.05)

time.sleep(1)
p = 0
for i in range(200):
    obj.fill((0, 0, 0))
    p += 0.05
    p %= 2 * np.pi

    obj.set_decreasing_intensity_merge_polar(0.4, [p], (0.5, 0.5), 0.2, (255, 0, 0))
    obj.set_decreasing_intensity_merge_polar(
        0.4, [(2 * np.pi) - p], (0.5, 0.5), 0.2, (0, 0, 255)
    )

    obj.show()
    # time.sleep(0.05)
