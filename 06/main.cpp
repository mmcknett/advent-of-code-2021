#include <iostream>
#include <sstream>
#include <string>
#include <iterator>
#include <vector>
#include <numeric>
using namespace std;

void readCommaSeparatedValues(uint64_t fish[]);
uint64_t countFish(uint64_t fish[]);
string counts_str(uint64_t fish[]);

const uint64_t SIZE = 9;

int main() {
  uint64_t fish[SIZE] = {0, 0, 0, 0, 0, 0, 0, 0, 0};
  const int simulateDays = 256;

  readCommaSeparatedValues(fish);

  for (int i = 0; i < simulateDays; ++i) {
    uint64_t newFish = fish[0];
    for (int f = 1; f < SIZE; ++f) {
      fish[f-1] = fish[f];
    }
    fish[8] = newFish;
    fish[6] += newFish;

    cout << "After " << i + 1 << " days: " << counts_str(fish) << " - " << countFish(fish) << " fish" << endl;
  }

  cout << "After " << simulateDays << " days there are " << countFish(fish) << " fish." << endl;

  return 0;
}

void readCommaSeparatedValues(uint64_t fish[]) {
  while(true) {
    int i;
    cin >> i;
    if (cin.peek() == ',') { cin.ignore(); }
    if (!cin.good()) { break; }

    cout << i << ' ';
    fish[i]++;
  }
  cout << endl;
}

uint64_t countFish(uint64_t fish[]) {
  uint64_t fishcount = 0;
  for (int i = 0; i < SIZE; ++i) {
    fishcount += fish[i];
  }
  return fishcount;
}

string counts_str(uint64_t fish[]) {
  ostringstream oss;
  copy(fish, fish + SIZE - 1, ostream_iterator<uint64_t>(oss, ","));
  oss << fish[SIZE - 1];
  return oss.str();
}
