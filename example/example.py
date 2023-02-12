#!/usr/bin/env python3

import light_arrangements_python
import time
import numpy as np

obj = light_arrangements_python.init_test(
    2, "./example/positions2d.csv", 0.03, (0.5, 0.5, 2), [0, 1, 2]
)

obj.set_closest_polar(0.1, [0], (0.5, 0.5), 0.2, (255, 0, 0))
obj.set_closest_polar(0.2, [np.pi], (0.5, 0.5), 0.2, (0, 255, 255))
obj.set_closest_polar(0.3, [np.pi / 2], (0.5, 0.5), 0.2, (255, 0, 255))
obj.set_closest_polar(0.4, [np.pi * 1.5], (0.5, 0.5), 0.2, (0, 0, 255))


obj.get_closest_polar(0.2, [40], (0.5, 0.5), 0.2)

for i in range(200):
    obj.show()
    time.sleep(0.05)
