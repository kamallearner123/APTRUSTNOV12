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

    // std::map <int, int> distances;
    // // {100: 50km}, {200: 70km}, 
    // std::map <int, vector<int>> trees;
    // // { 100: {400, 500, 300, 200}}

    // // 1) Find minimum spanning tree:
    // // Loop1: For 100, 200, 300, 400, 500
    // for (auto &p : cities){ 
    //     vector<int> new_path = {p.first};
    //     vector<int> visited_ndoes = {p.first};

    //     // Loop2: // Neighbours
    //     for (auto &neighbour: p.second)
    //     {
    //         cout << "Reference City: " << p.first << " Nighbours: " << neighbour.first << endl;
    //         //Loop3:
    //         // Go through all its neighbors and find nearest neighbour

    //         // keep adding nearest nodes in new_path
    //     }// Update distances
    // }// Loop1 : End

}