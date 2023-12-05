#include <cctype>
#include <iostream>
#include <ranges>

int main() {
  std::string token;
  int res = 0;

  auto is_alpha = [](char c) { return std::isalpha(c); };

  while (std::cin >> token) {
    auto forward = token | std::views::drop_while(is_alpha);
    auto backward =
        token | std::views::reverse | std::views::drop_while(is_alpha);
    int a = *forward.begin() - '0';
    int b = *backward.begin() - '0';

    res += a * 10 + b;
  }

  std::cout << res << "\n";
}
