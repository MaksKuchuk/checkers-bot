import game_checkers as gm
import pygame as pg
from controller import Controller
import tensorflow as tf

pg.init()
screen = pg.display.set_mode([800, 800])
cnt = Controller(screen)

model = tf.keras.models.load_model('models/model490.keras')

cnt.run_player2bot_game(model)
