import game_checkers as gm
import pygame as pg

pg.init()
screen = pygame.display.set_mode([500, 500])
xsize, ysize = screen.get_size()

running = True
while running: 
    for event in pygame.event.get(): 
        if event.type == pygame.QUIT: 
            running = False
    
