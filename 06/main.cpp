#include <iostream>
#include <sstream>
#include <string>
#include <iterator>
#include <vector>
#include <numeric>
using namespace std;

int main() {
  uint64_t fish[9] = {0, 0, 0, 0, 0, 0, 0, 0, 0};
  const int simulateDays = 256;

  while(true) {
    int i;
    cin >> i;
    if (cin.peek() == ',') {
      cin.ignore();
    }
    if (cin.eof()) {
      break;
    }
    cout << i << ' ';
    fish[i]++;
  }
  cout << endl;

  for (int i = 0; i < simulateDays; ++i) {
    uint64_t newFish = fish[0];
    for (int f = 1; f < 9; ++f) {
      fish[f-1] = fish[f];
    }
    fish[8] = newFish;
    fish[6] += newFish;

    uint64_t fishcount = 0;
    for (int i = 0; i < 9; ++i) {
      fishcount += fish[i];
    }

    ostringstream oss;
    copy(fish, fish + 8, ostream_iterator<uint64_t>(oss, ","));
    oss << fish[8];
    cout << "After\t" << i + 1 << " days:\t" << oss.str() << " " << fishcount << " fish" << endl;
  }

  uint64_t fishcount = 0;
  for (int i = 0; i < 9; ++i) {
    fishcount += fish[i];
  }
  cout << "After " << simulateDays << " days there are " << fishcount << " fish." << endl;

  return 0;
}