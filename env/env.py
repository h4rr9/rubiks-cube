from rubikscube import Cube, HalfTurnMetric, QuarterTurnMetric, Metric
import gym
import numpy as np

from gym import spaces


class CubeEnv(gym.Env):
    def __init__(self, metric: Metric, scramble_moves: int = 1000):

        super(CubeEnv, self).__init__()

        self.action_space = spaces.Discrete(metric.to_int())
        self.observation_space = spaces.MultiBinary(480)
        self.reward_range = spaces.Discrete(2)
        self.scramble_moves = scramble_moves

        self.cube = Cube.cube_htm(
        ) if metric == HalfTurnMetric else Cube.cube_qtm()
        self.cube.scramble(scramble_moves)

    def step(self, action: int) -> tuple[np.ndarray, float, bool, dict]:

        self.cube.turn(action)
        done: bool = self.cube.solved()
        return self.cube.representation(), int(done), done, {}

    def reset(self) -> tuple[np.ndarray, dict]:
        self.cube.scramble(self.scramble_moves)
        return self.cube.representation, {}

    def render(self):
        print(self.cube)
