#ifndef PIECE_H
#define PIECE_H

#define MAX_ROT 4
#define MAX_POS 9

//---------DIRECTIVES---------
#include <stdbool.h>

//---------DEFINITIONS---------

/*  3 = spade
 *  2 = diamond
 *  1 = heart
 *  0 = club
 * -1 = -heart
 * -2 = -diamond
 * -3 = -spade
 * -4 = -club

 * Rotation: 0 is two positive then 2 negative.
 * Each clockwise rotation adds 1

*/

typedef enum {
    TOP = 0,
    RIGHT = 1,
    BOTTOM = 2,
    LEFT = 3
} Side;

typedef enum {
    CLUB_SLOT = -4,
    SPADE_SLOT = -3,
    DIAMOND_SLOT = -2,
    HEART_SLOT = -1,
    CLUB_TAB = 0,
    HEART_TAB = 1,
    DIAMOND_TAB = 2,
    SPADE_TAB = 3
} Joint;

typedef struct {
    Joint * edge;
    int rotation;
    int index;
} Piece;

//----------PROTOTYPES---------

const char * side_to_str(Side s);
const char * joint_to_str(Joint j);
Joint get_joint(Piece* p, Side s);
void print_piece(Piece* p);
bool fit_joint(Joint j1, Joint j2);

#endif
