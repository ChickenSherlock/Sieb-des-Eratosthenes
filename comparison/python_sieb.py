from math import isqrt
import time


def basic_sieve(n: int):
    array = [True]*n
    array[0] = False
    array[1] = False

    for number in range(2,n):
        if array[number]:
            for i in range(number,n):
                if i % number == 0 and number != i:
                    array[i] = False

    return [i for i in range(n) if array[i]]


def improved_sieve(n: int):
    array = [True]*n
    array[0] = False
    array[1] = False

    for number in range(2,isqrt(n)):
        if array[number]:
            for i in range(number*number,n,number):
                array[i] = False

    return [i for i in range(n) if array[i]]


def main():
    n = 100
    basic_sieve(n)
    improved_sieve(n)


main()