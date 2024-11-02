# Pufferfish

Pufferfish is a compiler for a graphics programming language. 

[![Build and Test](https://github.com/cm226/PufferFish/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/cm226/PufferFish/actions/workflows/build-and-test.yml)

## Example program

A falling point example
```
fn falling(f){
  x = 315;
  y = f;
};

anim(falling);
```
<p align="center"><img src="./docs/example.gif" /></p>

### anim function

The anim function takes a function, that sets the y, x properties of the shape to animate.

## Runtime Requirements

* See Dockerfile

## Dev Requirements

* See Dockerfile
