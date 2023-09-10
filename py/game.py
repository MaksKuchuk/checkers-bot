import game_checkers as gm

def predictNextStep(model, game):
    steps = game.get_loc_possible_steps()
    maxres = -1
    maxstep = ((-1, -1), (-1, -1))

    for s in steps:
        res = model.predict([gm.input_vector_by_board(game.get_board(), game.get_order(), s)])
        if maxres < res:
            maxres = res
            maxstep = s

    return maxstep


def makeGame(model, lastSteps, amount):
    boards = []

    for i in range(amount):
        gameboards = []
        gamesteps = []
        g = gm.Checkers()
        c = 0
        f = False

        while True:
            gameboards.append(g.get_board)
            step = (predictNextStep(model, g), g.get_order())
            gamesteps.append(step)
            c += 1

            if not g.make_step(step[0][0], step[0][1]):
                raise NameError('step error')

            g.print_board()

            if g.is_win():
                break

            if c > 500:
                f = True
                break

        if f:
            continue

        boards.append((gameboards, gamesteps))

    return boards