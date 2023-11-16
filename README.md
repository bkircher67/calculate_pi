# calculate_pi

a simple test app to get an overview on the performance of a system by calculating PI (3.1415926) with a numerical approximation

## Implementations

- Rust
- python
- micropython (tbd)

## Results

System | Rust (debug) | Rust (release) | Python 
---|---|---|---
Apple M1 | 0.57 | 0.14 | 7.36
Raspberry Pi 5 | 1.52 | 0.23 
ESP32 

all values in sec.

## Details

### Raspberry Pi 5

#### Rust - debug

```text
Calculate Pi
 1 calculate pi(         10) = 3.0418396189294032   delta = 0.09975303466038987
 2 calculate pi(        100) = 3.1315929035585537   delta = 0.00999975003123943
 3 calculate pi(       1000) = 3.140592653839794    delta = 0.000999999749998981
 4 calculate pi(      10000) = 3.1414926535900345   delta = 9.99999997586265e-5
 5 calculate pi(     100000) = 3.1415826535897198   delta = 1.0000000073340232e-5
 6 calculate pi(    1000000) = 3.1415916535897743   delta = 1.0000000187915248e-6
 7 calculate pi(   10000000) = 3.1415925535897915   delta = 1.0000000161269895e-7
 8 calculate pi(  100000000) = 3.141592643589326    delta = 1.0000467121074053e-8
Time elapsed in calculate_pi() is: 1.520549427s
```

#### Rust - release

```text
Calculate Pi
 1 calculate pi(         10) = 3.0418396189294032   delta = 0.09975303466038987
 2 calculate pi(        100) = 3.1315929035585537   delta = 0.00999975003123943
 3 calculate pi(       1000) = 3.140592653839794    delta = 0.000999999749998981
 4 calculate pi(      10000) = 3.1414926535900345   delta = 9.99999997586265e-5
 5 calculate pi(     100000) = 3.1415826535897198   delta = 1.0000000073340232e-5
 6 calculate pi(    1000000) = 3.1415916535897743   delta = 1.0000000187915248e-6
 7 calculate pi(   10000000) = 3.1415925535897915   delta = 1.0000000161269895e-7
 8 calculate pi(  100000000) = 3.141592643589326    delta = 1.0000467121074053e-8
Time elapsed in calculate_pi() is: 232.083569ms
```

### Apple MacBook Pro M1 (2020 version)

#### Rust - debug

```text
Calculate Pi
 1 calculate pi(         10) = 3.0418396189294032   delta = 0.09975303466038987
 2 calculate pi(        100) = 3.1315929035585537   delta = 0.00999975003123943
 3 calculate pi(       1000) = 3.140592653839794    delta = 0.000999999749998981
 4 calculate pi(      10000) = 3.1414926535900345   delta = 9.99999997586265e-5
 5 calculate pi(     100000) = 3.1415826535897198   delta = 1.0000000073340232e-5
 6 calculate pi(    1000000) = 3.1415916535897743   delta = 1.0000000187915248e-6
 7 calculate pi(   10000000) = 3.1415925535897915   delta = 1.0000000161269895e-7
 8 calculate pi(  100000000) = 3.141592643589326    delta = 1.0000467121074053e-8
Time elapsed in calculate_pi() is: 572.765792ms
```

#### Rust - release

```text
Calculate Pi
 1 calculate pi(         10) = 3.0418396189294032   delta = 0.09975303466038987
 2 calculate pi(        100) = 3.1315929035585537   delta = 0.00999975003123943
 3 calculate pi(       1000) = 3.140592653839794    delta = 0.000999999749998981
 4 calculate pi(      10000) = 3.1414926535900345   delta = 9.99999997586265e-5
 5 calculate pi(     100000) = 3.1415826535897198   delta = 1.0000000073340232e-5
 6 calculate pi(    1000000) = 3.1415916535897743   delta = 1.0000000187915248e-6
 7 calculate pi(   10000000) = 3.1415925535897915   delta = 1.0000000161269895e-7
 8 calculate pi(  100000000) = 3.141592643589326    delta = 1.0000467121074053e-8
Time elapsed in calculate_pi() is: 144.606459ms
```

#### Python 3.9.15

```
          10 ... 3.0418396189294032
         100 ... 3.1315929035585537
        1000 ... 3.140592653839794
       10000 ... 3.1414926535900345
      100000 ... 3.1415826535897198
     1000000 ... 3.1415916535897743
    10000000 ... 3.1415925535897915
   100000000 ... 3.141592643589326
XXXXXXXXXXXX ... 3.14159265358979323846264338327950288419716939937510
done in 7.360185146331787s
```
