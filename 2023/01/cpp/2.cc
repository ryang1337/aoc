#include <algorithm>
#include <cctype>
#include <iostream>
#include <unordered_map>

std::unordered_map<std::string, int> d = {
    {"one", 1}, {"two", 2},   {"three", 3}, {"four", 4}, {"five", 5},
    {"six", 6}, {"seven", 7}, {"eight", 8}, {"nine", 9}};
std::unordered_map<std::string, int> rd = {
    {"eno", 1}, {"owt", 2},   {"eerht", 3}, {"ruof", 4}, {"evif", 5},
    {"xis", 6}, {"neves", 7}, {"thgie", 8}, {"enin", 9}};

int getFirstDigit(std::string s, std::unordered_map<std::string, int> &digits) {
  if (s.length() < 5) {
    for (int i = 0; i <= s.length(); i++) {
      if (i >= 3) {
        if (digits.count(s.substr(i - 3, 3))) {
          return digits[s.substr(i - 3, 3)];
        }
      }
      if (i >= 4) {
        if (digits.count(s.substr(i - 4, 4))) {
          return digits[s.substr(i - 4, 4)];
        }
      }
      if (std::isdigit(s[i])) {
        return std::stoi(s.substr(i, 1));
      }
    }
  }

  for (int i = 0; i < 3; i++) {
    if (std::isdigit(s[i])) {
      return std::stoi(s.substr(i, 1));
    }
  }

  if (digits.count(s.substr(0, 3))) {
    return digits[s.substr(0, 3)];
  }
  if (digits.count(s.substr(1, 3))) {
    return digits[s.substr(1, 3)];
  }
  if (digits.count(s.substr(0, 4))) {
    return digits[s.substr(0, 4)];
  }

  for (int i = 3; i < 5; i++) {
    if (std::isdigit(s[i])) {
      return std::stoi(s.substr(i, 1));
    }
  }

  std::string three = s.substr(1, 3);
  std::string four = s.substr(0, 4);
  std::string five = "a" + s.substr(0, 4);

  for (int i = 4; i < s.length(); i++) {
    three = three.substr(1) + s[i];
    four = four.substr(1) + s[i];
    five = five.substr(1) + s[i];
    if (std::isdigit(s[i])) {
      return std::stoi(s.substr(i, 1));
    }
    if (digits.count(three)) {
      return digits[three];
    }
    if (digits.count(four)) {
      return digits[four];
    }
    if (digits.count(five)) {
      return digits[five];
    }
  }

  return -1;
}

int main() {
  std::string token;
  int res = 0;

  while (std::cin >> token) {
    int a = getFirstDigit(token, d);
    std::reverse(begin(token), end(token));
    int b = getFirstDigit(token, rd);

    std::cout << a * 10 + b << "\n";

    res += a * 10 + b;
  }

  std::cout << res << "\n";
}
