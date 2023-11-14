#!/usr/bin/env python3

from argparse import ArgumentParser

max_len = 27001

def generate_zkllvm_input_from_file(file_path: str) -> (int, str):
    int_list = []

    with open(file_path, 'rb') as f:
        while (byte := f.read(1)):
            int_list.append(int.from_bytes(byte, 'big'))

    zkllvm_input = [ f'{{"int":{i}}}' for i in int_list ]

    
    actual_len = len(zkllvm_input)
    assert(actual_len <= max_len - 1 )

    padding_len = max_len - actual_len - 1
    padding_input = ['{"int":138}'] * padding_len

    final_input = [ f'{{"int":{actual_len}}}' ] + zkllvm_input + padding_input

    return ','.join(final_input)


def main() -> None:
    parser = ArgumentParser(
        prog='zkllvm input translator',
        description='zkllvm input translator',
    )

    parser.add_argument(
        'assumption', type=str, help='The path to the assumption file.'
    )

    parser.add_argument(
        'claim', type=str, help='The path to the claim file.'
    )

    parser.add_argument(
        'proof', type=str, help='The path to the proof file.'
    )

    args = parser.parse_args()

    assumption_arr = generate_zkllvm_input_from_file(args.assumption)
    claim_arr = generate_zkllvm_input_from_file(args.claim)
    proof_arr = generate_zkllvm_input_from_file(args.proof)

    input_str = (
                  f'[\n'
                  f'  {{"array": [{assumption_arr}]}},\n'
                  f'  {{"array": [{claim_arr}]}},\n'
                  f'  {{"array": [{proof_arr}]}}\n'
                  f']'
                )

    print(input_str)


if __name__ == '__main__':
    main()
