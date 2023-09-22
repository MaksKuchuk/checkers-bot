import game_checkers as gm
import pygame as pg
from controller import Controller

pg.init()
screen = pg.display.set_mode([800, 800])

cnt = Controller(screen)
cnt.run_player2player_game(screen)
