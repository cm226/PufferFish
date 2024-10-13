# Pufferfish

Pufferfish is a compiler for a graphics programming language. 

## Example program

A falling point example
```
fn falling(f){
  x = 315;
  y = f;
};

anim(falling);
```

### anim function

The anim function takes a function, that sets the y, x properties of the shape to animate.

## Runtime Requirements

* Nasm assembler (v 2.16 or later if you want debugging )
* gnu linker (ld)
* SDL2 

## Dev Requirements

* Rust + cargo
* gcc compiler
