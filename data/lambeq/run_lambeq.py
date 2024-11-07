from lambeq import BobcatParser
import json

parser = BobcatParser()
tree = parser.sentence2tree('This is a test sentence')
tree_json = json.dumps(tree.to_json())
with open ("data/ccg_parsed_sentence.json", "w") as f:
        f.write(tree_json)

