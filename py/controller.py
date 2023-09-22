from gui import Renderer
import game_checkers as gm
import pygame as pg

class Controller:
    def __init__(self, screen):
        self.game = gm.Checkers()
        self.handeled_checker_pos = None
        self.screen = screen

    def run_player2player_game(self, screen):
        running = True
        while running: 
            for event in pg.event.get(): 
                if event.type == pg.QUIT: 
                    running = False

            self.take_checker()
            self.place_checker()

            Renderer.update(self.screen, self.game, self.handeled_checker_pos, pg.mouse.get_pos())

    def run_player2bot_game(self, screen):
        running = True
        while running: 
            for event in pg.event.get(): 
                if event.type == pg.QUIT: 
                    running = False

            if self.game.get_order() == Order.WHITE:
                self.take_checker()
                self.place_checker()
            else:
                pass
                #self.game.make_step(step[0], step[1])

            Renderer.update(self.screen, self.game, self.handeled_checker_pos, pg.mouse.get_pos())

    def run_bot2bot_game(self, screen):
        pass

    def is_mouse_button_pressed(self):
        if pg.mouse.get_pressed()[0]:
            return True
        return False

    def is_mouse_button_released(self):
        if not pg.mouse.get_pressed()[0]:
            return True
        return False

    def take_checker(self):
        if not self.is_mouse_button_pressed():
            return

        pos = self.get_cell_by_pixel(pg.mouse.get_pos())
        if pos == None:
            return

        steps = self.game.get_loc_possible_steps()

        for p in steps:
            if (int(pos[0]), int(pos[1])) == p[0]:
                self.handeled_checker_pos = pos
                break
            
    def place_checker(self):
        if not self.is_mouse_button_released():
            return

        if self.handeled_checker_pos == None:
            return

        h_pos = (int(self.handeled_checker_pos[0]), int(self.handeled_checker_pos[1]))

        pos = self.get_cell_by_pixel(pg.mouse.get_pos())
        pos = (int(pos[0]), int(pos[1]))
        if pos == None:
            return

        steps = self.game.get_loc_possible_steps()

        if (h_pos, pos) in steps:
            self.game.make_step(h_pos, pos)
        
        self.handeled_checker_pos = None
    

    def get_cell_by_pixel(self, pos):
        h_cell = Renderer.get_cell_by_pixel(self.screen, self.game, pos)

        if (h_cell[0] < 0 or h_cell[1] < 0 or 
            h_cell[0] >= self.game.get_board_size()[0] or 
            h_cell[1] >= self.game.get_board_size()[1]):
            return None
        else:
            return h_cell[0], self.game.get_board_size()[1] - h_cell[1]