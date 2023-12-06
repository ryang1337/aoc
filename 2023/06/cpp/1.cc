#include <cctype>
#include <iostream>
#include <ranges>
#include <utility>
#include <vector>

std::vector<int> parse(std::string s) {
  std::vector<int> res;
  int i = 0;
  while (i < s.length()) {
    if (!std::isdigit(s[i])) {
      i++;
      continue;
    }

    int begin = i;
    while (i < s.length() && std::isdigit(s[i])) {
      i++;
    }
    int end = i;

    int num = std::stoi(s.substr(begin, end - begin));
    res.push_back(num);
  }

  return std::move(res);
}

int main() {
  std::string time;
  std::string distance;
  std::getline(std::cin, time);
  std::getline(std::cin, distance);
  std::vector<int> times = parse(time);
  std::vector<int> distances = parse(distance);
  std::vector<std::pair<int, int>> pairs;

  int res = 1;

  for (int i = 0; i < times.size(); i++) {
    int t = times[i];
    int d = distances[i];

    std::vector<int> v(t + 1);
    for (int j = 0; j < v.size(); j++) {
      v[j] = j;
    }

    int count = 0;

    auto ds = v | std::views::transform([=](int n) { return (t - n) * n; }) |
              std::views::drop_while([&count, d](int n) {
                bool lt = n <= d;
                if (lt) {
                  count++;
                }
                return lt;
              });
    int a = *ds.begin();
    res *= v.size() - (count * 2);
  }

  std::cout << res << "\n";
}
