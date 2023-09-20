import game_checkers as gm
import tensorflow as tf
import numpy as np
import game_checkers as gm

def fitModel(model, games):
    data = []
    res = []

    for game_round in games:
        winner = game_round[-1][-1][-1]

        for i in range(len(game_round[0])):
            data.append(gm.input_vector_by_board(game_round[0][i], game_round[1][i][1], game_round[1][i][0]))
            res.append(1. if winner == game_round[1][i][1] else 0.)

    model.fit(data, res, epochs=10)

    
