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

model = tf.keras.Sequential([
    tf.keras.layers.Dense(256, input_shape=(192,)),
    tf.keras.layers.Dense(256),
    tf.keras.layers.Dense(256),
    tf.keras.layers.Dense(1)
])

model.compile(optimizer='adam', loss='mse')

from game import *

boards = makeGame(model, 5, 1)

# print(boards)


