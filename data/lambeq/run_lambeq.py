import sys
import json
from lambeq import BobcatParser

# Ensure we get the sentence from the command line arguments.
if len(sys.argv) < 2:
    print("Usage: python run_lambeq.py <sentence>")
    sys.exit(1)

sentence = sys.argv[1]  # The sentence passed from Rust

parser = BobcatParser()
tree = parser.sentence2tree(sentence)  # Use the sentence argument
#diagram = parser.sentence2diagram(sentence)
#diagram.draw()
tree_json = json.dumps(tree.to_json())


# Write the JSON output to the file.
with open("data/temp_ccg_parsed_sentence.json", "w") as f:
    f.write(tree_json)
