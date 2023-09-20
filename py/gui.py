CELL_SIZE = 50.
CHECKER_SIZE = 40
STEP_SIZE = 15

BACKGROUND_CL = (0.974, 0.974, 0.974, 1.)

BLACK_CELL_CL = (0.772, 0.392, 0.203, 1.)
BHITE_CELL_CL = (0.913, 0.796, 0.647, 1.)

STEP_CL = (0., 0., 0., 1.)

CHECKER_BLACK_CL = (0.16, 0.16, 0.16, 1.)
CHECKER_WHITE_CL = (0.882, 0.854, 0.741, 1.)

import game_checkers as gm
from startGame import xsize, ysize
import pygame as pg

def get_screen_center():
    return xsize() / 2., ysize() / 2.

class Renderer:
    def draw_board(screen, game):
        start_pos = self.get_start_position()

        for x in range(game.get_board_size()[0]):
            for y in range(game.get_board_size()[1]):

                pg.draw.rect(
                        screen, 
                        BLACK_CELL_CL if (x + y) % 2 != 0 else WHITE_CELL_CL,
                        (start_pos[0] + x * CELL_SIZE, start_pos[1] + y * CELL_SIZE, CELL_SIZE, CELL_SIZE)
                    )

    def draw_checkers(screen, game, handeled_pos) {
        h_pos = (-1, -1) if handeled_pos == None else handeled_pos

        for x in range(game.get_board_size()[0]):
            for y in range(game.get_board_size()[1]):
                if (x, y) == h_pos:
                    continue
                
                Self::draw_checker_pos(screen, game, (x, y));

    def draw_checker_pos(screen, game, pos):
        if game.get_board().get_pos_color() == None:
            return

        col = game.get_board().get_pos_color()

        if game.get_board().get_pos_is_king(pos):
            draw_circle_lines_in_cell(screen, game, pos, col, CHECKER_SIZE, col)
        else:
            draw_circle_lines_in_cell(screen, game, pos, col, CHECKER_SIZE, col)

    def draw_circle_in_cell(screen, game, pos, size, col):
            start_pos = get_start_position(game);
            xpos = pos[0] * CELL_SIZE + CELL_SIZE / 2.;
            ypos = (game.get_board_size()[1] - pos[1] - 1) * CELL_SIZE + CELL_SIZE / 2.;
            pg.draw.circle(screen, col, (start_pos[0] + xpos, start_pos[1] + ypos), size / 2.)

    def draw_circle_lines_in_cell(screen, game, pos, size, col):
        start_pos = get_start_position(game);
        xpos = pos[0] * CELL_SIZE + CELL_SIZE / 2.;
        ypos = (game.get_board_size()[1] - pos[1] - 1) * CELL_SIZE + CELL_SIZE / 2.;
        pg.draw.circle(screen, col, (start_pos[0] + xpos, start_pos[1] + ypos), size / 2., size / 4.)
    
    def get_start_position(game):
        x, y = get_screen_center()

        return x - CELL_SIZE * game.get_board_size()[0] / 2., y - CELL_SIZE * game.get_board_size()[1] / 2.,




