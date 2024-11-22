#include <stdio.h>

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <stdio.h>

#define SCREEN_WIDTH 640
#define SCREEN_HEIGHT 480

SDL_Window* window = NULL;
SDL_Texture *img = NULL;
SDL_Renderer* renderer = NULL;

void gLibAssert(void* thing, char* msg) { 
  if(!thing){
    fprintf(stderr, msg);
    exit(1);
  }
}


SDL_Rect center() { 
  SDL_Rect r = {.x=SCREEN_WIDTH/2 , .y=SCREEN_HEIGHT/2, .h=0, .w=0};
  return r;
}

extern void clear() {
  gLibAssert(renderer, "Compiler bug :-(, Sorry this should never happen. Renderer failed to init");
  SDL_RenderClear(renderer);
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
  renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);
  
  return;
 }

extern void destroy_window(){
  SDL_DestroyWindow(window);
  SDL_DestroyTexture(img);
  SDL_DestroyRenderer(renderer);
  SDL_Quit();
}

extern void blit() { 
  gLibAssert(renderer, "Compiler bug :-(, Sorry this should never happen. Renderer failed to init");
  SDL_RenderPresent(renderer);
  clear();
}

extern int draw_shape(int x, int y, SDL_Texture* img) {

  gLibAssert(img, "Compiler bug :-(, Sorry this should never happen. The shape graphic has not been loaded");

  SDL_Rect pos = center();
  pos.x += x;
  pos.y += y;
  SDL_QueryTexture(img, NULL, NULL, &pos.w, &pos.h);
  SDL_RenderCopy(renderer, img, NULL, &pos);
  return 0;
}

extern SDL_Texture* loadImageTex(char* img) {
  gLibAssert(renderer, "Compiler bug :-(, Sorry this should never happen. Renderer failed to init");
  return IMG_LoadTexture(renderer, img);
}
