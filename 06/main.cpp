#include <iostream>
#include <sstream>
#include <string>
#include <iterator>
#include <vector>
using namespace std;

int main() {
  vector<int> fish;
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
    fish.push_back(i);
  }

  for (int i = 0; i < simulateDays; ++i) {
    int newFish = 0;
    for (int f = 0; f < fish.size(); ++f) {
      if (fish[f] == 0) {
        ++newFish;
        fish[f] = 6;
      } else {
        fish[f]--;
      }
    }
    for (int n = 0; n < newFish; ++n) {
      fish.push_back(8);
    }

    // ostringstream oss;
    // copy(begin(fish), end(fish) - 1, ostream_iterator<int>(oss, ","));
    // oss << fish.back();
    cout << "After\t" << i + 1 << " days:\t" << /*oss.str()*/ fish.size() << " fish" << endl;
  }

  cout << "After " << simulateDays << " days there are " << fish.size() << " fish." << endl;

  return 0;
}