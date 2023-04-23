#!/usr/bin/env python
import fire
import os
from typing import TypeAlias

PathType: TypeAlias = str
ROOT_RUST_CMD = "cargo"


def __run_cmds(args: list[str]) -> None:
    os.system(' '.join(args))


def run() -> None:
    args = [ROOT_RUST_CMD, "run"]
    __run_cmds(args)


if __name__ == '__main__':
    fire.Fire()
