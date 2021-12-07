#include <iostream>
#include <vector>
#include <sstream>
#include <string>
#include <numeric>
#include <limits>
#include <algorithm>
using namespace std;

void readCommaSeparatedValues(vector<int64_t>&);
int64_t sumDifferences(int64_t, const vector<int64_t>&);
int64_t sumCorrectedDifferences(int64_t, const vector<int64_t>&);

int main() {
  vector<int64_t> crabs;
  readCommaSeparatedValues(crabs);

  auto min_sum = numeric_limits<int64_t>::max();
  auto min_corrected = numeric_limits<int64_t>::max();
  auto min_pos = *min_element(begin(crabs), end(crabs));
  auto max_pos = *max_element(begin(crabs), end(crabs));

  for (auto pos = min_pos; pos <= max_pos; ++pos) {
    auto sum = sumDifferences(pos, crabs);
    auto corrected_sum = sumCorrectedDifferences(pos, crabs);
    cout << "Power if aligned on position " << pos << " is "
      << sum << "; corrected is "
      << corrected_sum << endl;
    min_sum = min(sum, min_sum);
    min_corrected = min(corrected_sum, min_corrected);
  }

  cout << "Minimum power is " << min_sum << endl;
  cout << "Corrected minimum power is " << min_corrected << endl;

  return 0;
}

void readCommaSeparatedValues(vector<int64_t>& crabs) {
  while(true) {
    int i;
    cin >> i;
    if (cin.peek() == ',') { cin.ignore(); }
    if (!cin.good()) { break; }

    cout << i << ' ';
    crabs.push_back(i);
  }
  cout << endl;
}

int64_t sumDifferences(int64_t pos, const vector<int64_t>& crabs) {
  return accumulate(begin(crabs), end(crabs), 0, [&pos](int64_t accum, int64_t next) {
    return accum + abs(next - pos);
  });
}

int64_t sumCorrectedDifferences(int64_t pos, const vector<int64_t>& crabs) {
  return accumulate(begin(crabs), end(crabs), 0, [&pos](int64_t accum, int64_t next) {
    auto absdiff = abs(next - pos);
    // 1 step is 1, 2 steps is 1 + 2, 3 steps is 1 + 2 + 3, ...
    return accum + absdiff * (absdiff + 1) / 2;
  });
}
