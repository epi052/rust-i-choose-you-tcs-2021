import random

import bit_flipper


def caveman_bit_flip_one(data):
    """ code taken from https://h0mbre.github.io/Fuzzing-Like-A-Caveman/# with author's permission """
    num_of_flips = int((len(data) - 4) * .01)

    indexes = range(4, (len(data) - 4))

    chosen_indexes = []

    # iterate selecting indexes until we've hit our num_of_flips number
    counter = 0
    while counter < num_of_flips:
        chosen_indexes.append(random.choice(indexes))
        counter += 1

    for x in chosen_indexes:
        current = data[x]
        current = (bin(current).replace("0b", ""))
        current = "0" * (8 - len(current)) + current

        indexes = range(0, 8)

        picked_index = random.choice(indexes)

        new_number = []

        # our new_number list now has all the digits, example: ['1', '0', '1', '0', '1', '0', '1', '0']
        for i in current:
            new_number.append(i)

        # if the number at our randomly selected index is a 1, make it a 0, and vice versa
        if new_number[picked_index] == "1":
            new_number[picked_index] = "0"
        else:
            new_number[picked_index] = "1"

        # create our new binary string of our bit-flipped number
        current = ''
        for i in new_number:
            current += i

        # convert that string to an integer
        current = int(current, 2)

        # change the number in our byte array to our new number we just constructed
        data[x] = current

    return data


def caveman_bit_flip_two(data):
    """ code taken from https://h0mbre.github.io/Fuzzing-Like-a-Caveman-2/ with author's permission """
    length = len(data) - 4

    num_of_flips = int(length * .01)

    picked_indexes = []

    flip_array = [1, 2, 4, 8, 16, 32, 64, 128]

    counter = 0
    while counter < num_of_flips:
        picked_indexes.append(random.choice(range(0, length)))
        counter += 1

    for x in picked_indexes:
        mask = random.choice(flip_array)
        data[x] = data[x] ^ mask

    return data


contents = bytearray(open('call-rust-from-python/corpus/Canon_40D.jpg', 'rb').read())


def test_caveman_bit_flip_one(benchmark):
    # benchmark.pedantic(caveman_bit_flip_one, (contents,), iterations=100000, rounds=10)
    benchmark(caveman_bit_flip_one, contents)


def test_caveman_bit_flip_two(benchmark):
    # benchmark.pedantic(caveman_bit_flip_two, (contents,), iterations=100000, rounds=10)
    benchmark(caveman_bit_flip_two, contents)


def test_rust_bit_flip(benchmark):
    # benchmark.pedantic(rust_exec.bit_flip, (contents,), iterations=100000, rounds=10)
    benchmark(bit_flipper.bit_flip, contents)
