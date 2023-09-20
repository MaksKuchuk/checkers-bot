import game_checkers as gm
import numpy as np
import random as rnd

def predictNextStep(model, game):
    steps = list(game.get_loc_possible_steps())
    res = model.predict([gm.input_vector_by_board(game.get_board(), game.get_order(), s) for s in steps], verbose=0)

    return steps[np.argmax(res)]


def makeGame(model, lastSteps=1, amount=1, rndSteps=0):
    boards = []

    while len(boards) < amount:
        gameboards = []
        gamesteps = []
        g = gm.Checkers()
        c = 0
        f = False

        for i in range(rndSteps):
            gameboards.append(g.get_board())
            step = (rnd.choice(list(g.get_loc_possible_steps())), g.get_order())
            gamesteps.append(step)

            if not g.make_step(step[0][0], step[0][1]):
                raise RuntimeError('step error')

            if g.is_win():
                break

        while True:
            gameboards.append(g.get_board())
            step = (predictNextStep(model, g), g.get_order())
            gamesteps.append(step)
            c += 1

            if not g.make_step(step[0][0], step[0][1]):
                raise RuntimeError('step error')

            if g.is_win():
                break

            if c > 150:
                f = True
                break

        if f:
            continue

        boards.append((gameboards[-lastSteps:], gamesteps[-lastSteps:]))

    return boards