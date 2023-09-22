CELL_SIZE = 100.
CHECKER_SIZE = 80
STEP_SIZE = 30

# BACKGROUND_CL = (0.974, 0.974, 0.974)

# BLACK_CELL_CL = (0.772, 0.392, 0.203)
# WHITE_CELL_CL = (0.913, 0.796, 0.647)

# STEP_CL = (0., 0., 0.)

# CHECKER_BLACK_CL = (0.16, 0.16, 0.16)
# CHECKER_WHITE_CL = (0.882, 0.854, 0.741)

BACKGROUND_CL = (248, 248, 248)

BLACK_CELL_CL = (196, 100, 52)
WHITE_CELL_CL = (233, 203, 165)

STEP_CL = (0, 0, 0)

CHECKER_BLACK_CL = (41, 41, 41)
CHECKER_WHITE_CL = (225, 217, 189)


import game_checkers as gm
import pygame as pg

def get_screen_center(screen):
    xsize, ysize = screen.get_size()
    return xsize / 2., ysize / 2.

def get_col(c):
    if c == gm.Order.WHITE:
        return CHECKER_WHITE_CL
    else:
        return CHECKER_BLACK_CL

class Renderer:
    def draw_board(screen, game):
        start_pos = Renderer.get_start_position(screen, game)

        for x in range(game.get_board_size()[0]):
            for y in range(game.get_board_size()[1]):

                pg.draw.rect(
                        screen, 
                        BLACK_CELL_CL if (x + y) % 2 != 0 else WHITE_CELL_CL,
                        (start_pos[0] + x * CELL_SIZE, start_pos[1] + y * CELL_SIZE, CELL_SIZE, CELL_SIZE)
                    )

    def draw_checkers(screen, game, handeled_pos):
        h_pos = (-1, -1) if handeled_pos == None else (int(handeled_pos[0]), int(handeled_pos[1]))

        for x in range(game.get_board_size()[0]):
            for y in range(game.get_board_size()[1]):
                if (x, y) == h_pos:
                    continue
                
                Renderer.draw_checker_pos(screen, game, (x, y));

    def draw_possible_steps(screen, game, handeled_pos):
        if handeled_pos == None:
            return

        h_pos = (int(handeled_pos[0]), int(handeled_pos[1]))

        steps = game.get_loc_possible_steps();

        for pos in steps:
            if h_pos == pos[0]:
                Renderer.draw_circle_in_cell(screen, game, pos[1], STEP_SIZE, STEP_CL)

    def draw_mouse(screen, game, handeled_pos, mouse_pos):
        if handeled_pos == None:
            return

        h_pos = (int(handeled_pos[0]), int(handeled_pos[1]))

        col = 0
        order_pos = game.get_board().get_pos_color(h_pos)
        if order_pos == gm.Order.WHITE:
            col = CHECKER_WHITE_CL
        elif order_pos == gm.Order.BLACK:
            col = CHECKER_BLACK_CL
        else:
            return

        if game.get_board().get_pos_is_king(h_pos):
            pg.draw.circle(screen, 
                           col, 
                           (mouse_pos[0], mouse_pos[1]), 
                           int(CHECKER_SIZE / 2.), 
                           int(CHECKER_SIZE / 4.))
        else:
            pg.draw.circle(screen, 
                           col, 
                           (mouse_pos[0], mouse_pos[1]), 
                           int(CHECKER_SIZE / 2.))

    def update(screen, game, handeled_pos, mouse_pos):
        screen.fill(BACKGROUND_CL)

        Renderer.draw_board(screen, game);
        Renderer.draw_checkers(screen, game, handeled_pos);
        Renderer.draw_possible_steps(screen, game, handeled_pos);
        Renderer.draw_mouse(screen, game, handeled_pos, mouse_pos);

        pg.display.update()
        pg.time.delay(10)

    def draw_checker_pos(screen, game, pos):
        col = game.get_board().get_pos_color(pos)
        if col == None:
            return

        if game.get_board().get_pos_is_king(pos):
            Renderer.draw_circle_lines_in_cell(screen, game, pos, CHECKER_SIZE, col)
        else:
            Renderer.draw_circle_in_cell(screen, game, pos, CHECKER_SIZE, col)

    def draw_circle_in_cell(screen, game, pos, size, col):
            start_pos = Renderer.get_start_position(screen, game);
            xpos = pos[0] * CELL_SIZE + CELL_SIZE / 2.;
            ypos = (game.get_board_size()[1] - pos[1] - 1) * CELL_SIZE + CELL_SIZE / 2.;
            pg.draw.circle(screen, get_col(col), (int(start_pos[0] + xpos), int(start_pos[1] + ypos)), int(size / 2.))

    def draw_circle_lines_in_cell(screen, game, pos, size, col):
        start_pos = Renderer.get_start_position(screen, game);
        xpos = pos[0] * CELL_SIZE + CELL_SIZE / 2.;
        ypos = (game.get_board_size()[1] - pos[1] - 1) * CELL_SIZE + CELL_SIZE / 2.;
        pg.draw.circle(screen, get_col(col), (int(start_pos[0] + xpos), int(start_pos[1] + ypos)), int(size / 2.), int(size / 4.))
    
    def get_start_position(screen, game):
        x, y = get_screen_center(screen)

        return x - CELL_SIZE * game.get_board_size()[0] / 2., y - CELL_SIZE * game.get_board_size()[1] / 2.,

    def get_cell_by_pixel(screen, game, pos):
        x, y = pos
        board_start = Renderer.get_start_position(screen, game)
        x_p = ((x - board_start[0]) / CELL_SIZE)
        y_p = ((y - board_start[1]) / CELL_SIZE)

        return x_p, y_p
    



