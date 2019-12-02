//----------DIRECTIVES---------
#include <stdio.h>
#include "piece.h"

//----------FUNCTIONS---------

const char* side_to_str(Side s) {
    switch(s) {
        case TOP:
            return "TOP";
        case RIGHT:
            return "RIGHT";
        case BOTTOM:
            return "BOTTOM";
        case LEFT:
            return "LEFT";
    }
}

const char* joint_to_str(Joint j) {

    switch(j) {
        case CLUB_SLOT:
            return "CS";
        case SPADE_SLOT:
            return "SS";
        case DIAMOND_SLOT:
            return "DS";
        case HEART_SLOT:
            return "HS";
        case CLUB_TAB:
            return "CT";
        case HEART_TAB:
            return "HT";
        case DIAMOND_TAB:
            return "DT";
        case SPADE_TAB:
            return "ST";
    }

}

Joint get_joint(Piece* p, Side s) {
    int a = (s - p->rotation) % MAX_ROT;
    if(a < 0) a += MAX_ROT;
    return p->edge[a];
}

void print_piece(Piece* p) {
    printf("%s,%s,%s,%s(%d), ", joint_to_str(p->edge[0]), joint_to_str(p->edge[1]),
               joint_to_str(p->edge[2]), joint_to_str(p->edge[3]), p->rotation);
}

bool fit_joint(Joint j1, Joint j2) {
    return (j1 == CLUB_SLOT && j2 == CLUB_TAB)
        || (j1 == CLUB_TAB && j2 == CLUB_SLOT)
        || j1 == -j2;
}

