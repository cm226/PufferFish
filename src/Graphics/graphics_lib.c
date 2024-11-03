#include <stdio.h>

#include <SDL2/SDL.h>
#include <stdio.h>

#define SCREEN_WIDTH 640
#define SCREEN_HEIGHT 480

SDL_Window* window = NULL;

SDL_Surface* screenSurface = NULL;

SDL_Rect center() { 
  SDL_Rect r = {.x=SCREEN_WIDTH/2 , .y=SCREEN_HEIGHT/2, .h=0, .w=0};
  return r;
}

extern void clear() {
  SDL_FillRect(screenSurface, NULL, SDL_MapRGB(screenSurface->format, 0x21, 0x21, 0x21));
}

extern void create_window(){
  if (SDL_Init(SDL_INIT_VIDEO) < 0) {
    fprintf(stderr, "could not initialize sdl2: %s\n", SDL_GetError());
    return;
  }
  window = SDL_CreateWindow(
          "Pufferfish",
          SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED,
          SCREEN_WIDTH, SCREEN_HEIGHT,
          SDL_WINDOW_SHOWN
          );
  if (window == NULL) {
    fprintf(stderr, "could not create window: %s\n", SDL_GetError());
    return;
  }
  screenSurface = SDL_GetWindowSurface(window);
  SDL_FillRect(screenSurface, NULL, SDL_MapRGB(screenSurface->format, 0x21, 0x21, 0x21));
  SDL_UpdateWindowSurface(window);
  return;
 }

extern void destroy_window(){
  SDL_DestroyWindow(window);
  SDL_Quit();
}

extern void blit() { 
  SDL_UpdateWindowSurface(window);
  clear();
}

extern int draw_shape(int x, int y) {
  SDL_Rect pos = center();
  pos.x += x;
  pos.y += y;
  pos.w = 10;
  pos.h = 10;
  SDL_FillRect(screenSurface, &pos, SDL_MapRGB(screenSurface->format, 0xff, 0xff, 0xff));
  return 0;
}
