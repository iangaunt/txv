#include <iostream>
#include <SDL2/SDL.h>
#include <bitset>

#include "cpu.h"
#include "display.h"

#include "systems/opcodes.h"
#include "tests/tests.h"

using std::cout;
using std::endl;

int main(int argc, char *argv[]) {
    display* dis = new display(160, 144, 3, "gameboy");
    cpu* c = new cpu();
    opcodes* op = new opcodes(c);

    tests* t = new tests();

    bool loaded = c->load_rom("C:/Users/ianga/Desktop/Codespaces/dmg-cpu/roms/tetris.gb");
    if (!loaded) return -1;

    bool quit = false;
    int k = 0;
    while (!quit && c->running && k < 20) {
        quit = dis->fetch_input();

        unsigned char opc = c->read();
        op->parse(opc);

        k++;
    }

    return 0;
}