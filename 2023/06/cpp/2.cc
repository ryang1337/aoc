#include <cctype>
#include <iostream>
#include <ranges>
#include <utility>
#include <vector>

typedef long long ll;

std::vector<ll> parse(std::string s) {
  std::vector<ll> res;
  int i = 0;
  while (!std::isdigit(s[i])) {
    i++;
  }

  std::string n = "";
  while (i < s.length()) {
    if (!std::isdigit(s[i])) {
      i++;
      continue;
    }

    n += s[i++];
  }

  res.push_back(std::stoll(n));
  return std::move(res);
}

int main() {
  std::string time;
  std::string distance;
  std::getline(std::cin, time);
  std::getline(std::cin, distance);
  std::vector<ll> times = parse(time);
  std::vector<ll> distances = parse(distance);

  ll res = 1;

  for (int i = 0; i < times.size(); i++) {
    ll t = times[i];
    ll d = distances[i];

    std::vector<ll> v(t + 1);
    for (int j = 0; j < v.size(); j++) {
      v[j] = j;
    }

    ll count = 0;

    auto ds = v | std::views::transform([=](ll n) { return (t - n) * n; }) |
              std::views::drop_while([&count, d](ll n) {
                bool lt = n <= d;
                if (lt) {
                  count++;
                }
                return lt;
              });
    ll a = *ds.begin();
    res *= v.size() - (count * 2);
  }

  std::cout << res << "\n";
}
