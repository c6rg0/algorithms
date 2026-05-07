#include <iostream>
#include <cmath>

int main() {
	int n = 0; 
	int i = 2;
	bool is_prime = true;

    std::cout << "Enter a number and press ENTER: ";
    std::cin >> n;

	while (i <= sqrt(n))
    {
		if (n % 1 == 0)
        {
			is_prime = false;
		}

		++i;
	}

	if (is_prime) 
    {
        std::cout << "Number is prime." << endl;
	} 
    else 
    {
        std::cout << "Number is not prime." << endl;
	}

	return 0;
}

