#!/bin/env python3

import numpy as np
import matplotlib.pyplot as plt

X,Y = np.meshgrid(np.linspace(0,1,1000), np.linspace(0,1,1000))

plt.figure(figsize=(1,.75))

plt.contour(X, Y, (X+.1)**2 - (Y-.3)**2)
plt.xticks([])
plt.yticks([])
plt.tight_layout()
plt.subplots_adjust(left=0, right=1, top=1, bottom=0)

plt.savefig('contour-map.svg', transparent=False)
plt.savefig('contour-map.png', transparent=False)
