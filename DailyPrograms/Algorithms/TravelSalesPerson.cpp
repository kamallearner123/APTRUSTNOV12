#include <iostream>
#include <string.h>
#include <map>
#include <array>


using namespace std;

struct neighbour {
    int city_id;
    int distance;
};

int main() 
{
    // city ids 100, 200, 300, 400, 500
    std::map<int, std::array<neighbour, 4>> cities = {
        {100, {{{200, 40}, {300, 32}, {400, 15}, {500, 20}}}},
        {200, {{{100, 40}, {300, 32}, {400, 15}, {500, 20}}}},
        {300, {{{100, 32}, {200, 40}, {400, 15}, {500, 20}}}},
        {400, {{{100, 15}, {200, 40}, {300, 32}, {500, 20}}}},
        {500, {{{100, 20}, {200, 40}, {300, 32}, {400, 15}}}}
    };

}