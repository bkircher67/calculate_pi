import time


def calculate_pi(n: int):
    pi = 0.0
    div = 1.0
    mult = 1.0
    for _ in range(n):
        pi += (4.0 / div) * mult
        div += 2.0
        mult *= -1.0
    return pi


def main():
    start = time.time()
    n = 10
    for _ in range(8):
        pi = calculate_pi(n)
        print(f"{n:12} ... {pi}")
        n *= 10
    stop = time.time()

    print("XXXXXXXXXXXX ... 3.14159265358979323846264338327950288419716939937510")
    print(f"done in {(stop - start)}s")

if __name__ == '__main__':
    main()