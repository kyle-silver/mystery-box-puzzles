import json

with open("curated.json", "r") as f:
    puzzles = json.load(f)

for (i, puzzle) in enumerate(puzzles):
    if i % 10_000 == 0:
        print(f"writing file {i}")
    with open(f"puzzles/{i}.json", "w") as f:
        json.dump(puzzle, f)