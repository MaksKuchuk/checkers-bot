import os
os.environ['TF_CPP_MIN_LOG_LEVEL'] = '2'

import game_checkers as gm
import tensorflow as tf
import numpy as np

# g.print_board()
# g.get_order()
# g.get_loc_possible_steps()
# g.make_step((0, 0), (0, 0))
# g.is_win()
# g.get_board()
# g.get_board_size()
# g.input_vector(gm.Order.WHITE, ((0, 0), (0, 0)))
# input_vector_by_board(PyBoard, Order, ((0, 0), (0, 0)))

INPUT_SIZE = 192
new = False

model = 0
if new:
    model = tf.keras.Sequential([
        tf.keras.layers.Dense(256, input_shape=(INPUT_SIZE,), activation='relu'),
        tf.keras.layers.Dense(128, activation='relu'),
        tf.keras.layers.Dense(1, activation='sigmoid'),
    ])

    model.compile(optimizer='adam', loss='mse', metrics=["accuracy"])
else:
    model = tf.keras.models.load_model('models/model140.keras')

from game import *
from modelFit import *

for i in range(141, 500):
    print(f"iteration: {i}")
    boards = makeGame(model, lastSteps=i, rndSteps=3)
    fitModel(model, boards)

    if i % 10 == 0:
        model.save(f'models/model{i}.keras')


