# Simple 2-player snake game

Game made with a [piston](https://github.com/PistonDevelopers/piston) engine.

<a href="url"><img src="https://raw.githubusercontent.com/Rqnsom/snakesss/main/assets/sneks.gif" align="center" height=auto width=100% border="black"></a>

Note: *It's really hard to capture a fun animation
when playing solo a two-player game.*

## About
This was done as an educational challenge as a one-day *hackathon*.
After watching this [video](https://www.youtube.com/watch?v=HCwMb0KslX8),
which sets up the coding ground for the basic snake game.

I wish I had more time to implement multiple gamestates (like *MainMenu,
ScoreScreen...*), but it is what it is.

Enjoy!

## Movement
- Red player: _arrow keys_
- Blue player: _WASD keys_

Every melon consumed lengthens the snake.
- How to win: **Avoid losing!**
- How to lose: **Hit the enemy snake**!
- How to make peace: **Frontal collision with the other snake** (ain't
  these snakes peculiar creatures...)

General strategy:
 - _The bigger your snake becomes, the higher the chance your opponent will
 not be able to avoid your elooongated snake body._

## How to run
```sh
$ cargo run    # Should do the trick (from the repo root directory)
```


