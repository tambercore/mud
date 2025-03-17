#!/usr/bin/env python3
"""
A REST API server that uses lambeq's BobcatParser to parse sentences into CCG trees.

Usage:
    To run the server:
        python server.py
    To run in test mode:
        python server.py --test
"""

import argparse
import json
import sys

from flask import Flask, request, jsonify
import lambeq

app = Flask(__name__)

# Initialize the parser once so it's reused across requests.
parser_instance = lambeq.BobcatParser()

def get_ccg_trees(sentences: list) -> list:
    """
    Parse multiple sentences using lambeq's BobcatParser and return their CCG trees.

    Args:
        sentences (list): A list of sentences (strings) to parse.

    Returns:
        list: A list of CCG tree representations in JSON format.
    """
    trees = parser_instance.sentences2trees(sentences)
    return [tree.to_json() for tree in trees]

@app.route('/sentences', methods=['POST'])
def parse_sentences():
    """
    REST API endpoint that accepts a JSON payload with a "sentences" field.
    It returns a JSON object containing the CCG trees for the given sentences.
    """
    try:
        # Parse JSON request
        data = request.get_json(force=True)
        sentences = data.get("sentences", [])
        ccg_trees = get_ccg_trees(sentences)
        return jsonify({"ccg_trees": ccg_trees})
    except Exception as error:
        # Optionally log the error for debugging
        print("Error processing request:", error)
        return jsonify({"error": str(error)}), 500

def run_server(host: str = "127.0.0.1", port: int = 20041):
    """
    Starts the Flask REST API server.
    """
    print(f"Server listening on http://{host}:{port}")
    app.run(host=host, port=port, debug=False)

if __name__ == "__main__":
    # Set up argument parsing to allow test mode
    parser_arg = argparse.ArgumentParser(
        description="REST API Server for parsing sentences into CCG trees."
    )
    parser_arg.add_argument(
        "--test", action="store_true", help="Run test mode instead of starting the server."
    )
    args = parser_arg.parse_args()

    if args.test:
        # Test mode: parse a couple of sentences and print the results
        print("Running test mode...")
        test_sentences = ["The cat sits on the mat.", "A dog barks loudly."]
        print("Local test results:", get_ccg_trees(test_sentences))
    else:
        # Start the Flask server
        run_server()
