#!/usr/bin/env python3

from argparse import ArgumentParser

def generate_lurk_input_from_file(file_path: str) -> (int, str):
    int_list = []

    with open(file_path, 'rb') as f:
        while (byte := f.read(1)):
            int_list.append(int.from_bytes(byte, 'big'))

    lurk_input = [ f'{i}' for i in int_list ]
    return ' '.join(lurk_input)


def main() -> None:
    parser = ArgumentParser(
        prog='Lurk input translator',
        description='Lurk input translator',
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
    
    parser.add_argument(
        'path_to_lib_lurk', type=str, help='The path to the lib.lurk file.'
    )

    args = parser.parse_args()

    assumption_arr = generate_lurk_input_from_file(args.assumption)
    claim_arr = generate_lurk_input_from_file(args.claim)
    proof_arr = generate_lurk_input_from_file(args.proof)
    
    path_to_lib_lurk = args.path_to_lib_lurk

    input_str = (
                  f'!(load "{path_to_lib_lurk}")\n'
                  f'\n'
                  f'!(def gamma_buffer \'({assumption_arr}))\n'
                  f'!(def claim_buffer \'({claim_arr}))\n'
                  f'!(def proof_buffer \'({proof_arr}))\n'
                  f'\n'
                  f';; We want to zk-prove this execution\n'
                  f'(verify gamma_buffer claim_buffer proof_buffer)\n'
                  f'!(prove)'
                )

    print(input_str)


if __name__ == '__main__':
    main()
