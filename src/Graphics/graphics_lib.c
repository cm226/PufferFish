#include <stdio.h>

#include <SDL2/SDL.h>
#include <stdio.h>

#define SCREEN_WIDTH 640
#define SCREEN_HEIGHT 480

SDL_Window* window = NULL;
SDL_Surface* screenSurface = NULL;

extern void create_window(){
  if (SDL_Init(SDL_INIT_VIDEO) < 0) {
    fprintf(stderr, "could not initialize sdl2: %s\n", SDL_GetError());
    return 1;
  }
  window = SDL_CreateWindow(
          "hello_sdl2",
          SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED,
          SCREEN_WIDTH, SCREEN_HEIGHT,
          SDL_WINDOW_SHOWN
          );
  if (window == NULL) {
    fprintf(stderr, "could not create window: %s\n", SDL_GetError());
    return 1;
  }
  screenSurface = SDL_GetWindowSurface(window);
  SDL_FillRect(screenSurface, NULL, SDL_MapRGB(screenSurface->format, 0x21, 0x21, 0x21));
  SDL_UpdateWindowSurface(window);
  return 0;
 }

extern void destroy_window(){
  SDL_Delay(2000);
  SDL_DestroyWindow(window);
  SDL_Quit();
}
extern void blit() { 
  SDL_UpdateWindowSurface(window);
}

extern int draw_shape(int x, int y) {
  printf("called %i, %i", x, y);
  return 0;
}
