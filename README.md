# 2D Platformer Game

A 2D adventure style platformer game inspired by games like LIMBO and Have A Nice Death. The game will have a simple objective (Point A to Point B) whilst avoiding enemies.

Authors: Alejandro Chavez (achav8)

UIUC CS128 Finals Honors Project.

# Introduction/Overview

This 2D game will be built using Bevy game engine. It will be important that I understand and implement an ECS process for my game as detailed [here](https://bevyengine.org/learn/book/getting-started/ecs/). 

I can begin by creating a sandbox using sprites from Time Fantasy asset pack which includes 80+ RPG Character sprites which I could either use for my actual game or just as place holders if I decide to make my own sprites in the future. I can also use Time Fantasy Dark Dimension Tile set to create a simple level. LDTK (Level Design Tool Kit) could also be a viable option to build a simple sandbox to start off with. This step in my project will be to work out game controls/collision detection/other game physics.

I’ll be using Rapier, a 2D physics engine library for collision detection. 

My game will also instantiate enemies throughout the game level. The AI for the game will be simple and will be tasked with “attacking the player sprite”.

# Challenges

This project will have various challenges. One of the most difficult parts of this project will be creating collision detection for my sprites to ensure that they don’t travel beyond the game level. Enemies within my game will also be a crucial component. I’ll need to ensure that enemies AI doesn’t make the game “too difficult” but substantial enough to add a layer of difficulty to the game. 

  
