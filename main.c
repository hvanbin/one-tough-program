//----------DIRECTIVES----------

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "piece.h"

//----------CONSTANTS----------

const int MAX_WIDTH = 3;
const int MAX_HEIGHT = 3;

//This is the puzzle itself to be solved. Order does not necessarily matter here.
char const JIGSAW[MAX_POS][MAX_ROT] = {
    {SPADE_TAB, SPADE_TAB, HEART_SLOT, CLUB_SLOT},//8
    {SPADE_TAB, DIAMOND_TAB, SPADE_SLOT, HEART_SLOT},//7
    {HEART_TAB, DIAMOND_TAB, DIAMOND_SLOT, HEART_SLOT},//6
    {DIAMOND_TAB, CLUB_TAB, CLUB_SLOT, DIAMOND_SLOT},//5
    {CLUB_TAB, HEART_TAB, SPADE_SLOT, HEART_SLOT},//4
    {HEART_TAB, SPADE_TAB, SPADE_SLOT, CLUB_SLOT}, //3
    {HEART_TAB, DIAMOND_TAB, CLUB_SLOT, CLUB_SLOT},//2
    {CLUB_TAB, HEART_TAB, DIAMOND_SLOT, CLUB_SLOT}, //1
    {SPADE_TAB, DIAMOND_TAB, HEART_SLOT, DIAMOND_SLOT}, //0
};

//----------FUNCTIONS---------

void print_solution(Piece** sol) {
    for(int i = 0; i < MAX_POS; i++) {
        if(i>0 && !(i%MAX_WIDTH)) {
            if(i != MAX_POS - 1) {printf(",");}
            printf("\n");
        }
        print_piece(sol[i]);
    }
}

#ifdef DEBUG
void print_part_solution(Piece** sol, int pos) {
    for(int i = 0; i <= pos; i++) {
        if(i>0 && !(i%MAX_WIDTH)) printf("\n");
        print_piece(sol[i]);
    }
    printf("\n");
}
#endif

bool fit_piece(Piece** sol, Piece* p0, int pos) {

    bool fit = false;
    bool fit4 = false;

    //top
    fit = (pos - MAX_WIDTH < 0);
    fit = fit || (fit_joint(get_joint(p0, TOP), get_joint(sol[pos - MAX_WIDTH], BOTTOM)));

    //left
    fit4 = (pos % MAX_WIDTH == 0);
    fit4 = fit4 || (fit_joint(get_joint(p0, LEFT), get_joint(sol[pos - 1], RIGHT)));

    return fit && fit4;
}

Piece** find_possibles(Piece** solution, int pos, int* counter, int* used) {

    Piece** matches = calloc((MAX_POS - pos) * MAX_ROT, sizeof(Piece*));
    //For every possible piece
    for(int i = 0; i < MAX_POS; i++) {
        //Be that piece not used
        if(!((1 << i) & *used)) {
            //Try all rotations
            for(int rot = MAX_ROT-1; rot >= 0; rot--) {

                Piece* p = malloc(sizeof(Piece));
                p->edge = malloc(sizeof(char)*MAX_ROT);
                //For each rotation, copy the side into the temporary piece from the given jigsaw puzzle.
                for(int j = MAX_ROT-1; j >= 0; j--)
                    p->edge[j] = JIGSAW[i][j];
                p->rotation = rot;
                p->index = i;
                if(fit_piece(solution, p, pos)) {
                    matches[(*counter)++] = p;
                }
            }
        }
    }
    return matches;
}

bool solve(Piece** sol, int pos, int* used) {

    Piece** possibles;
    /* Indicates if the puzzle has been solved. This boolean could likely be factored out cleanly.
       Once found is true, recursion will collapse.
    */
    bool found = false;
    int* counter = malloc(sizeof(int)); //which possible piece you are using
    *counter = 0;

    if(pos >= MAX_POS) {return true;}
    else {
        // This is the second bulk of the program, which generates matches for the next piece based on the left and above pieces.
        possibles = find_possibles(sol, pos, counter, used);
        while(!found && --(*counter) >= 0) {
            sol[pos] = possibles[*counter];
            #ifdef DEBUG
            print_part_solution(sol, pos);
            printf("...\n");
            #endif
            *used |= 1 << (sol[pos]->index);
            possibles[*counter] = NULL;
            //Recursive solving happens here.
            if(!(found = solve(sol, pos+1, used))) {
                if(sol[pos]) {
                    *used &= ~(1 << (sol[pos]->index));
                    sol[pos] = NULL;
                }
            }
        }
    }
    return found;
}

//----------DATA---------

Piece* solution[MAX_POS];

//This array contains up to all pieces at all their rotations at once.
Piece* possible[MAX_POS * MAX_ROT];

//---------ENTRY---------

int main(int argc, char* argv[]) {

    /* This keeps track of the current piece's index in the puzzle.
       The top-left is 0, and the bottom-right is the number of pieces in the puzzle - 1.
    */
    int position = 0;
    /* This is a bit flag used to track which positions are used (so pieces cannot be reused during recursion.
       This could equally (and probably) be more prettily passed as position is through the recursion.
    */
    int *used = malloc(sizeof(int)); *used = 0;

    //Here solve(...) enters the bulk of the program.
    if(solve(solution, position, used)) {
        print_solution(solution);
        printf("\n");
    }
    else {
        printf("No solution\n");
    }
}
