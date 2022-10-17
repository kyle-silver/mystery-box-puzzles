from collections import Counter
from typing import List
import json

def all_letters_sufficiently_used(jumble: str, words: List[str]) -> bool:
    all_words = "".join(words)
    frequencies = {
        c: 0
        for c in jumble
    }
    for word in words:
        for c in word:
            if c in frequencies.keys():
                frequencies[c] += 1;
    return all(v > 3 for v in frequencies.values())

print("loading")

with open("output.json", "r") as f:
    puzzles = json.load(f)

print("loaded")

curated = [
    puzzle
    for puzzle in puzzles
    if all_letters_sufficiently_used(puzzle["input"], puzzle["solutions"]) and len(puzzle["solutions"]) >= 11
]

with open("curated.json", "w") as f:
    json.dump(curated, f, indent=4)