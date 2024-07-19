#!/usr/bin/env python3
import os

def find_keep_sorted(directory, max_distance):
    occurrences = []

    # Walk through the directory and inspect each file
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith(".bazel"):
                file_path = os.path.join(root, file)
                with open(file_path, 'r') as f:
                    for line_num, line in enumerate(f, start=1):
                        if 'Keep sorted' in line:
                            occurrences.append((file_path, line_num))

    # Sort occurrences and calculate distances
    occurrences.sort()
    distances = [
        (file_path, line_num, occurrences[i + 1][1] - line_num)
        for i, (file_path, line_num) in enumerate(occurrences[:-1])
        if occurrences[i + 1][0] == file_path
    ]

    # Sort distances by the smallest distance first and print results
    for file_path, line_num, distance in sorted(distances, key=lambda x: x[2]):
        if distance <= max_distance:
            print(f"{file_path}:{line_num} distance {distance}")

if __name__ == "__main__":
    directory_to_search = "."
    max_distance = 5
    find_keep_sorted(directory_to_search, max_distance)
