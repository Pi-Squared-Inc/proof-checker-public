#!/usr/bin/env python3

from argparse import ArgumentParser
from pathlib import Path

max_len = 27001

def generate_zkllvm_input_from_file(file_path: str, suffix: str, fixed_size: int) -> (str, int):
    int_list = []

    filename = Path(file_path).stem + '.' + suffix
    input_file = Path(file_path).joinpath(filename)
    with open(input_file, 'rb') as f:
        while (byte := f.read(1)):
            int_list.append(int.from_bytes(byte, 'big'))

    zkllvm_input = [ f'{{"int":{i}}}' for i in int_list ]

    
    actual_len = len(zkllvm_input)
    padding_input = []

    if fixed_size is not None:
        assert(actual_len <= fixed_size - 1 )

        padding_len = fixed_size - actual_len - 1
        padding_input = ['{"int":138}'] * padding_len

    final_input = [ f'{{"int":{actual_len}}}' ] + zkllvm_input + padding_input

    return (','.join(final_input), actual_len)


def main() -> None:
    parser = ArgumentParser(
        description='Translate binary input to zkllvm input',
    )

    parser.add_argument(
        'input_dir', type=str, help='The input directory that contains binary input.'
    )

    parser.add_argument(
        '-o', type=str, dest='output_file', required=True,
        help='The file that stores the output.'
    )

    parser.add_argument(
        '--fixed_size', type=int, metavar='N',
        help='if provided, the input array will be of fixed size N'
    )

    args = parser.parse_args()

    assumption_arr, alen = generate_zkllvm_input_from_file(args.input_dir, 'ml-gamma', args.fixed_size)
    claim_arr, clen = generate_zkllvm_input_from_file(args.input_dir, 'ml-claim', args.fixed_size)
    proof_arr, plen = generate_zkllvm_input_from_file(args.input_dir, 'ml-proof', args.fixed_size)

    input_str = (
                  f'[\n'
                  f'  {{"array": [{assumption_arr}]}},\n'
                  f'  {{"array": [{claim_arr}]}},\n'
                  f'  {{"array": [{proof_arr}]}}\n'
                  f']'
                )

    with open(args.output_file, 'w') as f:
        f.write(input_str)

    print(f'{alen + 1}, {clen + 1}, {plen + 1}, {max(alen, clen, plen) + 1}')


if __name__ == '__main__':
    main()
