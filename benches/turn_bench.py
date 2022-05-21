import timeit

TRIALS = int(1e7)

t_turn_repr_solve = timeit.timeit(
    'cube.turn(0);cube.representation();cube.solved()',
    setup='from rubikscube import Cube;cube=Cube.cube_htm()',
    number=TRIALS)

t_turn_repr = timeit.timeit(
    'cube.turn(0);cube.representation()',
    setup='from rubikscube import Cube;cube=Cube.cube_htm()',
    number=TRIALS)

t_turn = timeit.timeit(
    'cube.turn(0)',
    setup='from rubikscube import Cube;cube=Cube.cube_htm()',
    number=TRIALS)

print(f"time -- turn + repr + solved :::  {t_turn_repr_solve}")
print(f"time -- turn + repr  :::  {t_turn_repr}")
print(f"time -- turn  :::  {t_turn}")
