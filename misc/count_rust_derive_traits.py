#!/usr/bin/env python3
import os
import re
from collections import defaultdict

def count_traits(directory):
    # Regex to match the derive traits
    derive_pattern = re.compile(r'#\[derive\((.*?)\)\]')
    
    trait_count = defaultdict(int)

    # Walk through the directory
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                with open(file_path, 'r') as f:
                    content = f.read()
                    
                    # Find all derive traits in the file
                    derives = derive_pattern.findall(content)
                    for derive in derives:
                        traits = [trait.strip() for trait in derive.split(',')]
                        for trait in traits:
                            trait_count[trait] += 1

    # Sort the traits by frequency in descending order
    sorted_traits = sorted(trait_count.items(), key=lambda x: x[1], reverse=True)
    
    # Print the sorted results
    for trait, count in sorted_traits:
        print(f'{trait}: {count}')

# Example usage
if __name__ == "__main__":
    directory = '.'
    count_traits(directory)

'''
Clone: 4572
PartialEq: 3753
Debug: 3527
Eq: 2225
Deserialize: 1594
Serialize: 1255
::prost::Message: 1185
Copy: 750
candid::Deserialize: 695
serde::Serialize: 694
candid::CandidType: 691
CandidType: 622
Hash: 549
comparable::Comparable: 546
Default: 376
PartialOrd: 312
Ord: 305
serde::Deserialize: 200
Parser: 154
::prost::Oneof: 132
Error: 60
ProposalMetadata: 51
thiserror::Error: 47
::prost::Enumeration: 43
EnumIter: 41
ZeroizeOnDrop: 41
Zeroize: 40
Decode: 40
Encode: 40
JsonSchema: 38
ValidateEq: 32
Args: 18
Template: 12
strum_macros::Display: 10
IntoStaticStr: 8
Arbitrary: 8
Display: 7
strum_macros::EnumIter: 7
EnumString: 6
Subcommand: 6
ExhaustiveSet: 6
ValueEnum: 5
std::hash::Hash: 5
AsRefStr: 5
std::fmt::Debug: 4
EnumCount: 4
Message: 4
ArgEnum: 4
VariantNames: 2
Request: 1
clap::ArgEnum: 1
EnumMessage: 1
Educe: 1
FromRepr: 1
: 1
strum_macros::EnumString: 1
clap::Args: 1
clap::Subcommand: 1
'''
