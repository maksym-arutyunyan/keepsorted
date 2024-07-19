#!/usr/bin/env python3
import os

def find_keep_sorted(directory):
    occurrences = []

    # Walk through the directory and inspect each file
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith(".bazel"):
                file_path = os.path.join(root, file)
                with open(file_path, 'r') as f:
                    lines = f.readlines()
                    for line_num, line in enumerate(lines, start=1):
                        if 'Keep sorted' in line:
                            occurrences.append((file_path, line_num))

    # Sort the occurrences by file path and line number
    occurrences.sort()

    # Calculate the line distances within each file
    distances = []
    current_file = None
    current_file_occurrences = []

    for file_path, line_num in occurrences:
        if file_path != current_file:
            if current_file_occurrences:
                calculate_distances(current_file_occurrences, distances)
            current_file = file_path
            current_file_occurrences = [(file_path, line_num)]
        else:
            current_file_occurrences.append((file_path, line_num))

    if current_file_occurrences:
        calculate_distances(current_file_occurrences, distances)

    # Sort distances by the smallest distance first
    distances.sort(key=lambda x: x[2])

    # Print the results
    for file_path, line_num, distance in distances:
        print(f"{file_path} at line {line_num} with distance {distance}")

def calculate_distances(occurrences, distances):
    for i in range(len(occurrences) - 1):
        file_path, line_num = occurrences[i]
        next_line_num = occurrences[i + 1][1]
        distance = next_line_num - line_num
        distances.append((file_path, line_num, distance))

if __name__ == "__main__":
    directory_to_search = "."
    find_keep_sorted(directory_to_search)

